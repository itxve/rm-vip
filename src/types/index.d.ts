export type JyJsonFile = {
  all_draft_store: Array<{
    draft_cover: string;
    draft_json_file: string;
    draft_name: string;
    tm_draft_create: number;
    tm_draft_modified: number;
  }>;
  root_path: string;
};
