---
type: Dataset
title: Customer Orders
description: Raw order data from the e-commerce platform.
tags:
  - ecommerce
  - raw
owner: Data Engineering
status: published
---

# Definition

Customer Orders dataset contains all orders placed on the platform.

# Schema

| Column | Type | Description |
|--------|------|-------------|
| order_id | string | Unique order identifier |
| customer_id | string | Customer identifier |
| order_date | timestamp | When the order was placed |
| total_amount | decimal | Order total in USD |
| status | string | Order status (pending, completed, cancelled) |

# Update frequency

Daily batch at 02:00 UTC.

# Source system

E-commerce platform database (PostgreSQL)