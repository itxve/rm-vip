export type RustCallResult<T> = {
  data: T;
  err: String;
};

export enum InitAppDataEnum {
  EXIST,
  CreateError,
  SUCCESS,
}
