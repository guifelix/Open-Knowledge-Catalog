---
type: Dataset
title: Customer Orders
description: Raw order data from the e-commerce platform.
tags:
  - sales
  - raw-data
  - e-commerce
owner: Data Engineering
status: published
---

# Schema

| Column | Type | Description |
|--------|------|-------------|
| order_id | string | Unique order identifier |
| customer_id | string | Customer identifier |
| order_date | date | Date of order |
| amount | decimal | Order total in USD |
| status | string | Order status (pending, completed, cancelled) |

# Notes

Data is updated daily at 6 AM UTC.