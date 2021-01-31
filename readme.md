# Search Engine Development Kit

This is an experimental library for an opinionated wrapper around elasticsearch that will handle many of the usual tasks of maintaining a search engine. None of it is implemented yet. This document describes what will hopefully exist.

## DSL for Elasticsearch Indices
**Status: In Progress**

Types with equality and hashing for describing index mappings.

## Derive Macros for Index Mapping DSL
**Status: Not Started**

Macros for generating index mappings based on a struct representing a record for that index.

## Alias Management
**Status: Not Started**

A standard search flow might look like this;
* Statically named alias for application access
* Timestamped hash named indices backing the alias
* Script to generate a new timestamped index
* Retention policy for indices to allow emergency rollback

None of these tasks are complex, but they are very inter related and involve a lot of boilerplate. By using versioned DSL representations of the index structure, we can automate most of this process and produce update and rollback scripts with minimal configuration.

## DSL for Elasticsearch Queries
**Status: Not Started**

There are often multiple complex queries in a search system, usually competing against each other in online experiments. Implementing our own DSL for these queries allows us to get the same versioning benefits as for index mappings, and also allows us to build unit tests that check that a query will be valid against a particular index mapping.

## Search Log Data Format
**Status: Not Started**

Search query logs together with logs of clickthrough or engagement are necessary to improve search relevance. Building a standard format for these logs allows us to provide implementations for common optimization tasks.

## Query Comparison
**Status: Not Started**

Once query log data annotated with the query used is available, we can compare queries to each other on a variety of performance metrics.

## Click Model Construction
**Status: Not Started**

Search query logs can be used to construct a probabalistic model that can be used as a replacement for user data in offline optimization

## Query Parameter Optimization
**Status: Not Started**
The simplest offline optimization task is sequential optimization for individual search queries. This allows simple tuning of the relative weights of various fields which would lack rigor and be extremely tedious to do by hand.
