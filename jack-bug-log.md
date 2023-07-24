causes of jack bugs

- failed to pass correct number of arguments which messed up stack. very difficult to find root cause.

- operator precedence.
  e.g. expected this
  return live_neighbours = 2 | live_neighbours = 3;
  to parse as
  return (live_neighbours = 2) | (live_neighbours = 3);
  whereas it actually parses as
  return (live_neighbours = (2 | live_neighbours)) = 3
