import { PostToken } from '../util/postTokenizer';

// TODO: There's probably a proper type exposed in vue, should find it later
export type Emit = (event: string, payload: any) => any;

export interface Image {
  id: number;
  name: string;
  path: string;
  preview_path: string;
}

export interface NewPost {
  name: string;
  message: string;
  file?: File;
}

export interface NewThread extends NewPost {
  title: string;
}

export interface Post {
  id: number;
  thread: number;
  name: string;
  timestamp: number;
  message: string;
  messageTokens: PostToken[];
  links: number[];
  image?: Image;
}
export interface Thread {
  id: number;
  board: string;
  title: string;
  open: boolean;
  posts: Post[];
}

export interface Board {
  code: string;
  name: string;
  description: string;
}
