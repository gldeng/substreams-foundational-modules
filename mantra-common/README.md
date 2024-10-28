# mantra-common

Common Mantra Substreams modules to extract events and transactions with indexing
This package inherits from the Injective Foundational Modules, which share the Cosmos Block model.

## Usage

To run the Substreams:

```bash
substreams build
substreams auth
substreams gui
```

## Modules

### all_events (map)

Retrieves all the events in the Mantra blockchain without any filtering.

### index_events (index)

The module create an index (a cache) to efficiently retrieve events by their type and/or attribute keys.

The module sets the keys corresponding to all event 'types' and 'attribute keys' in the block For example: `type:coin_received`, `attr:action`, `attr:sender` ...

The attribute values are never indexed because they have a high cardinality and would be too expensive to index.

### filtered_events (map)

The module reads from `all_events` and applies a filter on the event types and attribute keys, only outputing the events that match the filter.

The filter is specificed in the parameters of the module.

```yaml
...

params:
  filtered_events: "(type:rewards && attr:validator)"
```

### filtered_event_groups (map)

The module reads from `all_events` and applies a filter on the event types and attribute keys, outputing all the events from transactions that have at least one event matching the filter.

```yaml
params:
    filtered_event_groups: "type:rewards && attr:validator"
```

### filtered_events_by_attribute_value (map)

The module reads from `all_events` and applies a filter on the event types, attribute keys and values, only outputing the events that match the filter.

**NOTE:** This module does not use the index created by `index_events`.

```yaml
params:
    filtered_events_by_attribute_value: "type:rewards && attr:validator:mantravaloper18se5kq0z86pqfym8uuuqp77kyd788npj3wx7fc"
```

### filtered_event_groups_by_attribute_value (map)

The module reads from `all_events` and applies a filter on the event types, attribute keys and values, outputing all the events from transactions that have at least one event matching the filter.

**NOTE:** This module does not use the index created by `index_events`.

```yaml
params:
    filtered_events_by_attribute_value: "type:rewards && attr:validator:mantravaloper18se5kq0z86pqfym8uuuqp77kyd788npj3wx7fc"
```