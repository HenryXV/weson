use flexi_logger::{
    Age, Cleanup, Criterion, FileSpec, LogSpecification, Logger, LoggerHandle, Naming, WriteMode,
};

pub fn init_logger() -> anyhow::Result<LoggerHandle> {
    let logger = Logger::with(LogSpecification::parse("trace").unwrap())
        .log_to_file(FileSpec::default().directory("logs"))
        .rotate(
            Criterion::Age(Age::Hour),
            Naming::Numbers,
            Cleanup::KeepLogFiles(2),
        )
        .write_mode(WriteMode::Async)
        .start()?;

    Ok(logger)
}
