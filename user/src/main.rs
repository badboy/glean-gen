mod metrics;

fn main() {
    metrics::used::os.set("Windows");
}
