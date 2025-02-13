/**
 * @file variational.c
 * @author 315 incline sage
 * @date 2025-02-12
 * 
 * @copyright Copyright (c) 2025
 * 
 */

#include "solana_sdk.h"

enum {
    VARIATIONAL_ENTRYPOINT_NUM_ACCOUNTS = 3,
};

extern uint64_t entrypoint(const uint8_t *input)
{
    SolAccountInfo accounts[VARIATIONAL_ENTRYPOINT_NUM_ACCOUNTS];
    SolParameters entrypoint_parameters;
    entrypoint_parameters.ka = accounts;
    if (!sol_deserialize(input, &entrypoint_parameters, SOL_ARRAY_SIZE(accounts))) {
        return ERROR_INVALID_ARGUMENT;
    }

    sol_log("Starting Variational!\nCompute Units: ");
    sol_log_compute_units();

    sol_log_params(&entrypoint_parameters);
    return SUCCESS;
}
