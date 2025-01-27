use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::MetricExporter;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::Resource;

pub fn init_metrics(name: &str) {
    #[allow(clippy::expect_used)]
    let exporter = MetricExporter::builder()
        .with_tonic()
        .build()
        .expect("Unable to setup metrics exporter.");

    // let exporter = opentelemetry_stdout::MetricExporter::default();
    let reader =
        PeriodicReader::builder(exporter, opentelemetry_sdk::runtime::TokioCurrentThread).build();

    let meter_provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            name.to_string(),
        )]))
        .build();
    global::set_meter_provider(meter_provider.clone());
}
