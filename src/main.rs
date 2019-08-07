use rusoto_ce::CostExplorer;
use rusoto_cloudwatch::CloudWatch;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    region: Option<String>,
    service: Option<String>,
    linked_account: Option<String>,
    granularity: Option<String>,
    ce_metric_type: Option<String>,
    namespace: String,
    metric_name: String,
}

fn main() -> Result<(), failure::Error> {
    env_logger::try_init()?;
    lambda_runtime::lambda!(handler);
    Ok(())
}

fn handler(event: Event, _: lambda_runtime::Context) -> Result<(), lambda_runtime::error::HandlerError> {
    let percentage = fetch_percentage(&event)?;
    if percentage.is_none() {
        let message = "There are no metrics".to_string();
        log::info!("{}", message);
        return Ok(());
    }
    put_metric_data(percentage.unwrap(), event)?;
    Ok(())
}

fn fetch_percentage(event: &Event) -> Result<Option<f64>, failure::Error> {
    let mut filter = rusoto_ce::Expression {
        and: Some(vec![]),
        ..Default::default()
    };
    push_filter(&event.region, filter.and.as_mut().unwrap(), "REGION".to_string());
    push_filter(&event.service, filter.and.as_mut().unwrap(), "SERVICE".to_string());
    push_filter(&event.linked_account, filter.and.as_mut().unwrap(), "LINKED_ACCOUNT".to_string());
    let ce_metric_utilization = "utilization".to_string();
    let ce_metrice_type = event.ce_metric_type.as_ref().unwrap_or(&ce_metric_utilization).as_str();
    match ce_metrice_type {
        "utilization" => fetch_utilization_percentage(&filter, &event),
        "coverage" => fetch_coverage_percentage(&filter, &event),
        _ => return Err(failure::format_err!("Unsupported Cost Explorer metrics type: {}", ce_metrice_type)),
    }
}

fn push_filter(event_element: &Option<String>, exps: &mut Vec<rusoto_ce::Expression>, key: String) {
    if event_element.is_some() {
        exps.push(rusoto_ce::Expression {
            dimensions: Some(rusoto_ce::DimensionValues {
                key: Some(key),
                values: Some(vec![event_element.clone().unwrap()]),
            }),
            ..Default::default()
        });
    }
}

fn fetch_utilization_percentage(filter: &rusoto_ce::Expression, event: &Event) -> Result<Option<f64>, failure::Error> {
    // CostExplorer API is available in only us-east-1 (https://ce.us-east-1.amazonaws.com/)
    let cost_explorer = rusoto_ce::CostExplorerClient::new(rusoto_core::Region::UsEast1);
    let today: chrono::Date<chrono::Utc> = chrono::Utc::today();
    let ce_request = rusoto_ce::GetReservationUtilizationRequest {
        filter: Some(filter.clone()),
        granularity: event.granularity.clone(),
        time_period: rusoto_ce::DateInterval {
            start: (today - chrono::Duration::days(7)).format("%Y-%m-%d").to_string(),
            end: today.format("%Y-%m-%d").to_string(),
        },
        ..Default::default()
    };

    log::info!("Make a request for Cost Explorer");
    log::info!("{:?}", ce_request);

    match cost_explorer.get_reservation_utilization(ce_request).sync() {
        Ok(r) => {
            if r.utilizations_by_time.last().is_some() {
                let percentage = r
                    .utilizations_by_time
                    .last()
                    .unwrap()
                    .total
                    .as_ref()
                    .unwrap()
                    .utilization_percentage
                    .as_ref()
                    .unwrap()
                    .parse()
                    .unwrap();
                Ok(Some(percentage))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(failure::format_err!("{:?}", e)),
    }
}

fn push_dimension(event_element: &Option<String>, dimensions: &mut Vec<rusoto_cloudwatch::Dimension>, name: String) {
    if event_element.is_some() {
        dimensions.push(rusoto_cloudwatch::Dimension {
            name: name,
            value: event_element.clone().unwrap(),
        });
    }
}

fn fetch_coverage_percentage(filter: &rusoto_ce::Expression, event: &Event) -> Result<Option<f64>, failure::Error> {
    // CostExplorer API is available in only us-east-1 (https://ce.us-east-1.amazonaws.com/)
    let cost_explorer = rusoto_ce::CostExplorerClient::new(rusoto_core::Region::UsEast1);
    let today: chrono::Date<chrono::Utc> = chrono::Utc::today();
    let ce_request = rusoto_ce::GetReservationCoverageRequest {
        filter: Some(filter.clone()),
        granularity: event.granularity.clone(),
        time_period: rusoto_ce::DateInterval {
            start: (today - chrono::Duration::days(7)).format("%Y-%m-%d").to_string(),
            end: today.format("%Y-%m-%d").to_string(),
        },
        ..Default::default()
    };

    log::info!("Make a request for Cost Explorer");
    log::info!("{:?}", ce_request);

    match cost_explorer.get_reservation_coverage(ce_request).sync() {
        Ok(r) => {
            if r.coverages_by_time.last().is_some() {
                let percentage = r
                    .coverages_by_time
                    .last()
                    .unwrap()
                    .total
                    .as_ref()
                    .unwrap()
                    .coverage_hours
                    .as_ref()
                    .unwrap()
                    .coverage_hours_percentage
                    .as_ref()
                    .unwrap()
                    .parse()
                    .unwrap();
                Ok(Some(percentage))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(failure::format_err!("{:?}", e)),
    }
}

fn put_metric_data(percentage: f64, event: Event) -> Result<String, failure::Error> {
    let cloudwatch = rusoto_cloudwatch::CloudWatchClient::new(rusoto_core::Region::default());
    let mut dimensions = vec![];
    push_dimension(&event.region, &mut dimensions, "Region".to_string());
    push_dimension(&event.service, &mut dimensions, "Service".to_string());
    push_dimension(&event.linked_account, &mut dimensions, "LinkedAccount".to_string());
    let metric_data = vec![rusoto_cloudwatch::MetricDatum {
        metric_name: event.metric_name,
        value: Some(percentage),
        dimensions: Some(dimensions),
        ..Default::default()
    }];
    let cw_metric_input = rusoto_cloudwatch::PutMetricDataInput {
        namespace: event.namespace,
        metric_data: metric_data,
    };

    log::info!("Make a request for CloudWatch Metrics");
    log::info!("{:?}", cw_metric_input);

    match cloudwatch.put_metric_data(cw_metric_input).sync() {
        Ok(r) => return Ok(format!("{:?}", r)),
        Err(e) => return Err(failure::format_err!("{:?}", e)),
    }
}
