---
title: "Customer Orders Dataset"
description: "Transactional order data for customers"
type: "Dataset"
tags: ["transactions", "customers", "revenue", "operational"]
owner: "data-engineering-team"
status: "active"
format: "parquet"
location: "s3://data-lake/customer-orders/"
schema_version: "2.1"
refresh_schedule: "daily"
---

# Customer Orders Dataset

## Overview

Contains all customer order transactions including subscriptions, one-time purchases, and refunds.

## Schema

| Column | Type | Description |
|--------|------|-------------|
| order_id | string | Unique order identifier |
| customer_id | string | Customer identifier |
| product_id | string | Product identifier |
| amount_usd | decimal | Order amount in USD |
| currency | string | Currency code |
| order_date | timestamp | Order timestamp |
| status | enum | pending, completed, cancelled, refunded |

## Related Concepts

- [Monthly Recurring Revenue](../metrics/mrr.md) - Derived metric
- [Customer Lifetime Value](../metrics/ltv.md) - Derived metric
- [Billing Transactions](../datasets/billing-transactions.md) - Related dataset