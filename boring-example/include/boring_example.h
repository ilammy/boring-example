#ifndef BORING_EXAMPLE_H
#define BORING_EXAMPLE_H

#include <stdbool.h>
#include <stdint.h>

bool compute_sha_256(
    const uint8_t *restrict input,
    size_t input_len,
    uint8_t *restrict output,
    size_t *restrict output_len
);

#endif /* BORING_EXAMPLE_H */
