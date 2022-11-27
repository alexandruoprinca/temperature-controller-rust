use mysql::prelude::Queryable;

use super::{ReadConfig, Config};

pub struct ConfigSqlReader {
    pool: mysql::Pool,
}

impl ConfigSqlReader {
    pub fn build(sql_connection_string: String) -> Result<Self, Box<dyn std::error::Error>> {
        let opts = mysql::Opts::from_url(&sql_connection_string)?;
        let pool = mysql::Pool::new(opts)?;
        Ok(ConfigSqlReader { pool })
    }
}


impl ReadConfig for ConfigSqlReader {
    fn get_config(&self) -> Result<Option<Config>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        let result = conn.query_map(
            r#"SELECT min_temperature, max_temperature FROM Config"#,
            |(min_temperature, max_temperature)| Config {
                min_temperature,
                max_temperature,
            },
        )?;
        if result.len() != 0 {
            return Ok(Some(result[0]));
        }
        Ok(None)
    }
}
