---
title: "Marketing Attribution Dataset"
description: "Marketing touchpoint attribution data"
type: "Dataset"
tags: ["marketing", "attribution", "acquisition", "operational"]
owner: "marketing-analytics-team"
status: "active"
format: "parquet"
location: "s3://data-lake/marketing-attribution/"
schema_version: "1.0"
refresh_schedule: "daily"
---

# Marketing Attribution Dataset

## Overview

Tracks customer touchpoints across marketing channels for attribution modeling.

## Schema

| Column | Type | Description |
|--------|------|-------------|
| touchpoint_id | string | Unique touchpoint identifier |
| customer_id | string | Customer identifier |
| channel | string | Marketing channel (paid_search, organic, email, social, etc.) |
| campaign_id | string | Campaign identifier |
| cost_usd | decimal | Cost of touchpoint |
| timestamp | timestamp | Touchpoint timestamp |
| is_converting | boolean | Whether this touchpoint led to conversion |

## Related Concepts

- [Customer Acquisition Cost](../metrics/cac.md) - Derived metric
- [Customer Orders](../datasets/customer-orders.md) - Conversion data