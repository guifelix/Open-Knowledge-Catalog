---
type: Metric
title: Customer Count
description: Total number of active customers.
tags:
  - finance
  - customer
owner: Finance Analytics
status: published
---

# Definition

Number of unique customers with at least one active subscription.

# Calculation

SELECT COUNT(DISTINCT customer_id) FROM subscriptions WHERE status = 'active'