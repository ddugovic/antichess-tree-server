#include <stdio.h>
#include <errno.h>
#include <string.h>

#include "tree.h"

int main() {
    tree_t tree;

    if (!tree_open(&tree, "easy18.done")) {
        printf("could not open tree: %s\n", strerror(errno));
        return 1;
    }

    query_result_t result;
    query_result_clear(&result);

    move_t *moves = { move_parse("e2e3") };
    size_t moves_len = 1;

    if (!tree_query(&tree, moves, moves_len, &result)) {
        printf("query failed\n");
        return 1;
    }

    // Could also query other trees now. Results are merged.

    query_result_sort(&result);

    for (size_t i = 0; i < result.num_children; i++) {
        char uci[MAX_UCI];
        move_uci(result.moves[i], uci);

        printf("%s %d\n", uci, result.sizes[i]);
    }

    return 0;
}
