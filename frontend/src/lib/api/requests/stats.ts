import { emptyRequest, type CallbacksSet } from "../api";
import { API_STATS_USERS_COUNT } from "../configs";

// Get users count
type GetUsersCountResponse = {
  count: number
};

export const getUsersCount = (
  callbacks: CallbacksSet<GetUsersCountResponse, [
  "FATAL_ERROR",
  "INTERNAL_SERVER_ERROR"
]>
) => emptyRequest(API_STATS_USERS_COUNT, "GET", callbacks);
