//! Small ordered scratch without a database's minimum disk allocation.
use super::*;
use redb::TableHandle as _;
use std::collections::BTreeMap;

const MEMORY_LIMIT: u64 = 64 * 1024 * 1024;
type Row = (Vec<u8>, Vec<u8>);

struct Value {
    bytes: Vec<u8>,
    weight: u64,
    _lease: super::super::memory::Lease,
}
#[derive(Default)]
pub(super) struct Small {
    tables: BTreeMap<String, BTreeMap<Vec<u8>, Value>>,
    bytes: u64,
}
impl Small {
    fn get(&self, table: Table, key: &[u8]) -> Option<Vec<u8>> {
        self.tables
            .get(table.name())?
            .get(key)
            .map(|v| v.bytes.clone())
    }
    fn put(&mut self, table: Table, key: &[u8], value: &[u8]) -> Result<()> {
        // Charge before key/value copies and BTree growth. Include tree nodes,
        // table framing and the bounded copies returned by ordered seeks.
        let weight = 256u64
            .checked_add((key.len() as u64).checked_mul(2).ok_or(Error::Limit)?)
            .and_then(|n| n.checked_add((value.len() as u64).checked_mul(2)?))
            .ok_or(Error::Limit)?;
        let next = self
            .bytes
            .checked_add(weight)
            .filter(|n| *n <= MEMORY_LIMIT)
            .ok_or(Error::Limit)?;
        let lease = super::super::memory::Lease::reserve(weight)?;
        let previous = self
            .tables
            .entry(table.name().to_owned())
            .or_default()
            .insert(
                key.to_vec(),
                Value {
                    bytes: value.to_vec(),
                    weight,
                    _lease: lease,
                },
            );
        self.bytes = next - previous.map_or(0, |v| v.weight);
        Ok(())
    }
    fn remove(&mut self, table: Table, key: &[u8]) {
        if let Some(value) = self
            .tables
            .get_mut(table.name())
            .and_then(|t| t.remove(key))
        {
            self.bytes -= value.weight;
        }
    }
    fn row(&self, table: Table, last: bool, after: Option<&[u8]>) -> Option<Row> {
        let table = self.tables.get(table.name())?;
        let row = if let Some(key) = after {
            table
                .range::<[u8], _>((std::ops::Bound::Excluded(key), std::ops::Bound::Unbounded))
                .next()
        } else if last {
            table.last_key_value()
        } else {
            table.first_key_value()
        };
        row.map(|(k, v)| (k.clone(), v.bytes.clone()))
    }
}

pub(super) enum Tables {
    Disk(Disk),
    Small(Small),
}
impl Tables {
    pub fn new(path: &Path, space: Arc<Space>, small: bool) -> Result<Self> {
        if small {
            Ok(Self::Small(Small::default()))
        } else {
            Disk::new(path, space).map(Self::Disk)
        }
    }
    pub fn get(&self, table: Table, key: &[u8]) -> Result<Option<Vec<u8>>> {
        match self {
            Self::Disk(d) => d.get(table, key),
            Self::Small(d) => Ok(d.get(table, key)),
        }
    }
    pub fn put(&mut self, table: Table, key: &[u8], value: &[u8]) -> Result<()> {
        match self {
            Self::Disk(d) => d.put(table, key, value),
            Self::Small(d) => d.put(table, key, value),
        }
    }
    pub fn remove(&mut self, table: Table, key: &[u8]) -> Result<()> {
        match self {
            Self::Disk(d) => d.remove(table, key),
            Self::Small(d) => {
                d.remove(table, key);
                Ok(())
            }
        }
    }
    pub fn first(&self, table: Table) -> Result<Option<Row>> {
        match self {
            Self::Disk(d) => d.first(table),
            Self::Small(d) => Ok(d.row(table, false, None)),
        }
    }
    pub fn last(&self, table: Table) -> Result<Option<Row>> {
        match self {
            Self::Disk(d) => d.last(table),
            Self::Small(d) => Ok(d.row(table, true, None)),
        }
    }
    pub fn after(&self, table: Table, key: &[u8]) -> Result<Option<Row>> {
        match self {
            Self::Disk(d) => d.after(table, key),
            Self::Small(d) => Ok(d.row(table, false, Some(key))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_tables_preserve_order_and_reject_growth_before_retaining_data() {
        let mut small = Small::default();
        let table = TableDefinition::new("test");
        for n in [2u32, 0, 1] {
            small.put(table, &n.to_be_bytes(), &[n as u8]).unwrap();
        }
        assert_eq!(small.row(table, false, None).unwrap().0, 0u32.to_be_bytes());
        assert_eq!(small.row(table, true, None).unwrap().0, 2u32.to_be_bytes());
        assert_eq!(
            small
                .row(table, false, Some(&0u32.to_be_bytes()))
                .unwrap()
                .0,
            1u32.to_be_bytes()
        );
        let before = small.bytes;
        assert_eq!(
            small
                .put(table, b"oversize", &vec![0; MEMORY_LIMIT as usize])
                .unwrap_err(),
            Error::Limit
        );
        assert_eq!(small.bytes, before);
        assert!(small.get(table, b"oversize").is_none());
        small
            .put(table, &1u32.to_be_bytes(), b"replacement")
            .unwrap();
        for n in 0u32..3 {
            small.remove(table, &n.to_be_bytes());
        }
        assert_eq!(small.bytes, 0);
    }
}
