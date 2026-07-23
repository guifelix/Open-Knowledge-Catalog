---
title: "Monthly Recurring Revenue (MRR)"
description: "Total predictable revenue from subscriptions"
type: "Metric"
tags: ["finance", "revenue", "subscription", "executive"]
owner: "finance-team"
status: "active"
unit: "USD"
frequency: "monthly"
source: "billing-system"
---

# Monthly Recurring Revenue (MRR)

## Definition

Monthly Recurring Revenue represents the total predictable revenue generated from active subscriptions in a given month.

## Calculation

```
MRR = SUM(monthly_subscription_value) for all active subscriptions
```

## Recognition Rules

- Only includes active, non-cancelled subscriptions
- Excludes one-time fees and setup charges
- Prorated for mid-month changes

## Related Concepts

- [Customer Acquisition Cost](../cac.md) - Cost to acquire revenue
- [Customer Lifetime Value](../ltv.md) - Long-term value
- [Churn Rate](../churn-rate.md) - Revenue at risk
- [Billing System Dataset](../../datasets/billing-transactions.md) - Source data