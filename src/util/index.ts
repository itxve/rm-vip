import {
  writeTextFile,
  readTextFile,
  BaseDirectory,
  exists,
} from "@tauri-apps/api/fs";
import { appConfigDir } from "@tauri-apps/api/path";

import { invoke } from "@tauri-apps/api/tauri";

const APP_CONFIG_FILE_NAME = "app.config.json";
const JY_FILE_NAME = "root_meta_info";

export { JY_FILE_NAME, APP_CONFIG_FILE_NAME };

import { RustCallResult } from "@/types";

/**
 *读取配置文件
 * @returns
 */
export async function readConfig() {
  const config = await readTextFile(APP_CONFIG_FILE_NAME, {
    dir: BaseDirectory.AppConfig,
  });
  return JSON.parse(config);
}

/**
 * 写入配置文件
 * @param data 文件内容
 */
export async function writeConfig(data: string) {
  await writeTextFile(APP_CONFIG_FILE_NAME, data, {
    dir: BaseDirectory.AppConfig,
  });
}

/**
 * 系统文件读取为数组
 * @param path
 * @returns
 */
export async function readSysFileForArray(path: String) {
  const file: RustCallResult<number[]> = await invoke("read_file", {
    filePath: path,
  });
  return file;
}

/**
 * 写入系统文件
 * @param filePath 系统文件目录
 * @param content 文件内容
 */
export async function writeSysFileFromString(
  filePath: string,
  content: string
) {
  const rt = await invoke<RustCallResult<String>>("write_file", {
    filePath,
    data: content,
  });
  return !rt.err;
}

export function sleep(ms: number) {
  return new Promise((fn) => setTimeout(fn, ms));
}
