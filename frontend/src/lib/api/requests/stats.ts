import { emptyRequest, type CallbacksSet } from "../api";
import { API_STATS_USERS_COUNT } from "../configs";


/* --------------- *
 * Get users count *
 * --------------- */ 

type GetUsersCountResponse = {
  /**
   * The total number of active users.
   */
  count: number
};

/**
 * Fetches the total number of users registered in the system.
 *
 * @param callbacks - The set of success and error callbacks.
 */
export const getUsersCount = async(
  callbacks: CallbacksSet<GetUsersCountResponse, [
  "FATAL_ERROR",
  "INTERNAL_SERVER_ERROR"
]>
) => emptyRequest(API_STATS_USERS_COUNT, "GET", callbacks);
