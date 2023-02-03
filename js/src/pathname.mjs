import { match, parse } from 'reghex';

const zip = x => ({ [x.tag]: x[0] });
const merge = x => x.reduce((prev, curr) => ({ ...prev, ...curr }), {});

const dash = match('dash')`
  ${/\-/}
`;

const slash = match('slash')`
  ${/\//}
`;

const letters = match('letters')`
  ${/[a-z\d]/}
`;

const pctEncoded = match('ptc_encoded')`
  ${/(%[A-F\d]{2})/}
`;

// ISO 3166-1 Country Codes
const country = match('country', zip)`
  ${/[A-Z]{2}/} :${slash}
`;

// ISO 639-1
const lang = match('lang', zip)`
  ${/[a-z]{2}/} :${slash}
`;

const contentType = match('contentType', zip)`
  ${/[a-z]{2,}/} :${slash}
`;

const titlePart = match('title_part', x => x.join(''))`
  (${letters} | ${pctEncoded})+
`;

const title = match('title', x => ({ title: decodeURIComponent(x.join('-')) }))`
  (${titlePart}+ :${dash})+
`;

const id = match('id', zip)`
  ${/[a-zA-Z\d]{8,13}/}
`;

const slug = match('slug', merge)`
  ${title}? ${id} :${slash}?
`;

const data = match('data', zip)`
  ${/[a-zA-Z\d]+/} :${slash}?
`;

const pathname = match('pathname', merge)`
  :${slash}
  ${country}?
  ${lang}?
  ${contentType}
  ${slug}
  ${data}?
`;

export const parsePathname = parse(pathname);
