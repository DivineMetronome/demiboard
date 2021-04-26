export const enum TokenType {
  Text,
  Bold,
  Italic,
  StrikeThrough,
  Code,
  Spoiler,
  Quote,
  LineBreak,
  PostLink,
  ThreadLink,
}

const rules = {
  [TokenType.Spoiler]: {
    openChar: '[',
    open: /\[spoiler\]/y,
    openLength: 9,
    close: /\[\/spoiler\]/y,
    closeLength: 10,
  },
  [TokenType.Bold]: {
    openChar: '[',
    open: /\[b\]/y,
    openLength: 3,
    close: /\[\/b\]/y,
    closeLength: 4,
  },
  [TokenType.Italic]: {
    openChar: '[',
    open: /\[i\]/y,
    openLength: 3,
    close: /\[\/i\]/y,
    closeLength: 4,
  },
  [TokenType.StrikeThrough]: {
    openChar: '[',
    open: /\[s\]/y,
    openLength: 3,
    close: /\[\/s\]/y,
    closeLength: 4,
  },
  [TokenType.Quote]: {
    openChar: '>',
    open: />/y,
    openLength: 1,
    close: /\n/y,
    closeLength: 1,
  },
};

const patterns = {
  [TokenType.Code]: {
    pattern: /\[code\]([\s\S]*)\[\/code\]\n?/y,
    first: '[',
  },
  [TokenType.LineBreak]: {
    pattern: /\n/y,
    first: '\n',
  },
  [TokenType.ThreadLink]: {
    pattern: />>>(\/[a-z]+\/\d+)/y,
    first: '>',
  },
  [TokenType.PostLink]: {
    pattern: />>(\d+)/y,
    first: '>',
  },
};

function prepareRule(type, rule) {
  const match = (input, position, pattern) => {
    pattern.lastIndex = position;
    const match = pattern.exec(input);

    return !(match == null || match.index !== position);
  };
  return {
    type: parseInt(type),
    openLength: rule.openLength,
    closeLength: rule.closeLength,
    open(input, position) {
      return match(input, position, rule.open);
    },
    close(input, position) {
      return match(input, position, rule.close);
    },
  };
}
const openByFirst = {};
for (const [type, rule] of Object.entries(rules)) {
  const prepared = prepareRule(type, rule);
  if (!openByFirst[rule.openChar]) {
    openByFirst[rule.openChar] = [];
  }
  openByFirst[rule.openChar].push(prepared);
}

function preparePattern(type, pattern) {
  return {
    type: parseInt(type),
    match(input, position) {
      pattern.lastIndex = position;
      const match = pattern.exec(input);
      if (match == null || match.index !== position) {
        return null;
      }

      return {
        length: match[0].length,
        value: match[1],
      };
    },
  };
}
const patternsByFirst = {};
for (const [type, { pattern, first }] of Object.entries(patterns)) {
  const prepared = preparePattern(type, pattern);
  if (!patternsByFirst[first]) {
    patternsByFirst[first] = [];
  }
  patternsByFirst[first].push(prepared);
}

function tokenize(input, links) {
  return {
    pos: 0,
    open: null,
    textStart: null,

    [Symbol.iterator]() {
      return this;
    },

    next() {
      main: while (this.pos < input.length) {
        let increment = 1;
        const first = input[this.pos];

        if (this.open) {
          const matches = this.open.rule.close(input, this.pos);
          if (matches) {
            increment = this.open.rule.closeLength;
            this.open.count--;
          }
        } else {
          const patterns = patternsByFirst[first] || [];

          for (const pattern of patterns) {
            const match = pattern.match(input, this.pos);

            if (!match) {
              continue;
            }

            if (this.textStart !== null) {
              break main;
            }
            const oldPos = this.pos;

            this.pos += match.length;
            if (pattern.type === TokenType.PostLink) {
              match.value = parseInt(match.value);
              links.push(match.value);
            }

            return {
              value: {
                idx: oldPos,
                type: pattern.type,
                value: match.value,
              },
              done: false,
            };
          }
        }
        const openRules = openByFirst[first] || [];

        for (const rule of openRules) {
          if (this.open && this.open.rule !== rule) continue;
          const matches = rule.open(input, this.pos);

          if (!matches) {
            continue;
          }
          if (this.textStart !== null) {
            break main;
          }

          increment = 0;
          this.pos += rule.openLength;

          if (!this.open) {
            this.open = {
              count: 1,
              start: this.pos,
              rule,
            };
          } else {
            this.open.count++;
          }
        }

        if (!this.open && this.textStart === null) {
          this.textStart = this.pos;
        }

        this.pos += increment;

        if (this.open?.count === 0) {
          const start = this.open.start;
          const end = this.pos - this.open.rule.closeLength;
          const type = this.open.rule.type;
          this.open = null;
          return {
            value: {
              idx: start,
              type,
              value: [...tokenize(input.substring(start, end), links)],
            },
            done: false,
          };
        }
      }

      if (this.open) {
        const start = this.open.start;
        const type = this.open.rule.type;
        this.open = null;
        return {
          value: {
            idx: start,
            type,
            value: [...tokenize(input.substring(start, this.pos), links)],
          },
          done: false,
        };
      }

      if (this.textStart !== null) {
        const start = this.textStart;
        this.textStart = null;
        return {
          value: {
            idx: start,
            type: TokenType.Text,
            value: input.substring(start, this.pos),
          },
          done: false,
        };
      }
      return { done: true };
    },
  };
}

export interface PostToken {
  type: TokenType;
  value: string | PostToken[];
  idx: number;
}
interface PostText {
  tokens: PostToken[];
  links: number[];
}
export default function parse(text: string): PostText {
  const links = [];
  const tokens = [...tokenize(text, links)];
  const uniqueLinks = [...new Set(links)];

  return {
    tokens,
    links: uniqueLinks.map(parseInt),
  };
}
