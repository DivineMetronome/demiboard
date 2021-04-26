import { h, resolveComponent, Component } from 'vue';
import { PostToken, TokenType } from '../../util/postTokenizer';
import Quote from './Quote.vue';
import Code from './Code.vue';
import Spoiler from './Spoiler.vue';
import PostLink from './PostLink.vue';
import { Post } from '../../types';
import { ReplyMap } from '../../util/useUpdatedThread';

interface Props {
  token: PostToken;
  posts: Post[];
  replyMap: ReplyMap;
}
const components = {
  [TokenType.Bold]: 'b',
  [TokenType.Italic]: 'i',
  [TokenType.StrikeThrough]: 's',
  [TokenType.Quote]: Quote,
  [TokenType.Spoiler]: Spoiler,
};

export default function (props: Props): Component | string {
  const Me = resolveComponent('text-fragment');
  switch (props.token.type) {
    case TokenType.Text: {
      return props.token.value as string;
    }
    case TokenType.Bold:
    case TokenType.Italic:
    case TokenType.StrikeThrough:
    case TokenType.Quote:
    case TokenType.Spoiler: {
      const Component = components[props.token.type];
      return (
        <Component>
          {(props.token.value as PostToken[]).map((token) => {
            return (
              <Me
                key={token.idx}
                token={token}
                posts={props.posts}
                reply-map={props['reply-map']}
              />
            );
          })}
        </Component>
      );
    }
    case TokenType.LineBreak:
      return <br />;
    case TokenType.PostLink: {
      return (
        <PostLink
          post={props.token.value}
          posts={props.posts}
          reply-map={props['reply-map']}
        />
      );
    }
    case TokenType.Code: {
      return <Code>{props.token.value}</Code>;
    }
    default:
      return 'missing token type ' + props.token.type;
  }
}
