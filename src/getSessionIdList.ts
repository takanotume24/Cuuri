import { invoke } from "@tauri-apps/api/core";
import { SessionId } from "./types";

export async function getSessionIdList(): Promise<SessionId[] | null> {
  try {
    const sessionIdList: SessionId[] = await invoke("get_session_id_list");

    return sessionIdList;
  } catch (error) {
    console.error("Failed to get session id list: ", error);
    return null;
  }
}
