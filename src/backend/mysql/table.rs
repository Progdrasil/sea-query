use super::*;

impl TableBuilder for MysqlQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut SqlWriter) {
        column_def.name.prepare(sql, '`');

        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            self.prepare_column_type(column_type, sql);
        }

        for column_spec in column_def.spec.iter() {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut SqlWriter) {
        write!(
            sql,
            "{}",
            match column_type {
                ColumnType::Char(length) => match length {
                    Some(length) => format!("char({})", length),
                    None => "char".into(),
                },
                ColumnType::String(length) => match length {
                    Some(length) => format!("varchar({})", length),
                    None => "varchar(255)".into(),
                },
                ColumnType::Text => "text".into(),
                ColumnType::TinyInteger(length) => match length {
                    Some(length) => format!("tinyint({})", length),
                    None => "tinyint".into(),
                },
                ColumnType::SmallInteger(length) => match length {
                    Some(length) => format!("smallint({})", length),
                    None => "smallint".into(),
                },
                ColumnType::Integer(length) => match length {
                    Some(length) => format!("int({})", length),
                    None => "int".into(),
                },
                ColumnType::BigInteger(length) => match length {
                    Some(length) => format!("bigint({})", length),
                    None => "bigint".into(),
                },
                ColumnType::Float(precision) => match precision {
                    Some(precision) => format!("float({})", precision),
                    None => "float".into(),
                },
                ColumnType::Double(precision) => match precision {
                    Some(precision) => format!("double({})", precision),
                    None => "double".into(),
                },
                ColumnType::Decimal(precision) => match precision {
                    Some((precision, scale)) => format!("decimal({}, {})", precision, scale),
                    None => "decimal".into(),
                },
                ColumnType::DateTime(precision) => match precision {
                    Some(precision) => format!("datetime({})", precision),
                    None => "datetime".into(),
                },
                ColumnType::Timestamp(precision) => match precision {
                    Some(precision) => format!("timestamp({})", precision),
                    None => "timestamp".into(),
                },
                ColumnType::TimestampWithTimeZone(precision) => match precision {
                    Some(precision) => format!("timestamp({})", precision),
                    None => "timestamp".into(),
                },
                ColumnType::Time(precision) => match precision {
                    Some(precision) => format!("time({})", precision),
                    None => "time".into(),
                },
                ColumnType::Date => "date".into(),
                ColumnType::Interval(_, _) => "unsupported".into(),
                ColumnType::Binary(length) => match length {
                    Some(length) => format!("binary({})", length),
                    None => "blob".into(),
                },
                ColumnType::Boolean => "bool".into(),
                ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => format!("money({}, {})", precision, scale),
                    None => "money".into(),
                },
                ColumnType::Json => "json".into(),
                ColumnType::JsonBinary => "json".into(),
                ColumnType::Uuid => "binary(16)".into(),
                ColumnType::Custom(iden) => iden.to_string(),
            }
        )
        .unwrap()
    }

    fn prepare_column_spec(&self, column_spec: &ColumnSpec, sql: &mut SqlWriter) {
        match column_spec {
            ColumnSpec::Null => write!(sql, "NULL"),
            ColumnSpec::NotNull => write!(sql, "NOT NULL"),
            ColumnSpec::Default(value) => write!(sql, "DEFAULT {}", self.value_to_string(value)),
            ColumnSpec::AutoIncrement => write!(sql, "AUTO_INCREMENT"),
            ColumnSpec::UniqueKey => write!(sql, "UNIQUE"),
            ColumnSpec::PrimaryKey => write!(sql, "PRIMARY KEY"),
            ColumnSpec::Extra(string) => write!(sql, "{}", string),
        }
        .unwrap()
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut SqlWriter) {
        let alter_option = match &alter.alter_option {
            Some(alter_option) => alter_option,
            None => panic!("No alter option found"),
        };
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            table.prepare(sql, '`');
            write!(sql, " ").unwrap();
        }
        match alter_option {
            TableAlterOption::AddColumn(column_def) => {
                write!(sql, "ADD COLUMN ").unwrap();
                self.prepare_column_def(column_def, sql);
            }
            TableAlterOption::ModifyColumn(column_def) => {
                write!(sql, "MODIFY COLUMN ").unwrap();
                self.prepare_column_def(column_def, sql);
            }
            TableAlterOption::RenameColumn(from_name, to_name) => {
                write!(sql, "RENAME COLUMN ").unwrap();
                from_name.prepare(sql, '`');
                write!(sql, " TO ").unwrap();
                to_name.prepare(sql, '`');
            }
            TableAlterOption::DropColumn(column_name) => {
                write!(sql, "DROP COLUMN ").unwrap();
                column_name.prepare(sql, '`');
            }
        }
    }

    fn prepare_table_rename_statement(&self, rename: &TableRenameStatement, sql: &mut SqlWriter) {
        write!(sql, "RENAME TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            from_name.prepare(sql, '`');
        }
        write!(sql, " TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            to_name.prepare(sql, '`');
        }
    }
}
