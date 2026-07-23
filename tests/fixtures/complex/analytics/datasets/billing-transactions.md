---
title: "Billing Transactions Dataset"
description: "Raw billing system transaction log"
type: "Dataset"
tags: ["billing", "transactions", "financial", "operational"]
owner: "data-engineering-team"
status: "active"
format: "parquet"
location: "s3://data-lake/billing-transactions/"
schema_version: "1.3"
refresh_schedule: "hourly"
---

# Billing Transactions Dataset

## Overview

Raw transaction log from the billing system including charges, credits, refunds, and adjustments.

## Schema

| Column | Type | Description |
|--------|------|-------------|
| transaction_id | string | Unique transaction identifier |
| customer_id | string | Customer identifier |
| subscription_id | string | Subscription identifier |
| amount_usd | decimal | Transaction amount in USD |
| type | enum | charge, credit, refund, adjustment |
| status | enum | succeeded, failed, pending |
| created_at | timestamp | Transaction timestamp |

## Related Concepts

- [Monthly Recurring Revenue](../metrics/mrr.md) - Derived metric
- [Churn Rate](../metrics/churn-rate.md) - Derived metric
- [Customer Orders](../datasets/customer-orders.md) - Related dataset