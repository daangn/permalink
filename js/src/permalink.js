import slugify from 'cjk-slug';
import { parsePathname } from './pathname.js';

const wellKnownOriginToCountry = {
  'https://daangn.com': 'KR',
  'https://karrotmarket.com': undefined,
  'https://www.daangn.com': 'KR',
  'https://www.karrotmarket.com': undefined,
  'https://ca.karrotmarket.com': 'CA',
  'https://jp.karrotmarket.com': 'JP',
  'https://uk.karrotmarket.com': 'UK',
  'https://us.karrotmarket.com': 'US',
  'https://kr.karrotmarket.com': 'KR',
};

const wellKnownCountryToOrigin = {
  'CA': 'https://ca.karrotmarket.com',
  'JP': 'https://jp.karrotmarket.com',
  'UK': 'https://uk.karrotmarket.com',
  'US': 'https://us.karrotmarket.com',
  'KR': 'https://www.daangn.com',
};

const wellKnownCountryToLanguage = {
  'CA': 'en',
  'JP': 'ja',
  'KR': 'ko',
  'UK': 'en',
  'US': 'en',
};

const aliases = {
  'https://daangn.com': 'https://www.daangn.com',
  'https://karrotmarket.com': 'https://www.karrotmarket.com',
};

function ensureTrailingSlash(pathLike) {
  return pathLike.endsWith('/') ? pathLike : pathLike + '/';
}

export function parse(urlLike) {
  const url = new URL(urlLike);
  url.pathname = ensureTrailingSlash(url.pathname);

  const { href, origin, pathname } = url;

  const {
    country = wellKnownOriginToCountry[origin] || wellKnownOriginToCountry[aliases[origin]],
    lang = wellKnownCountryToLanguage[country],
    contentType,
    title = null,
    id,
    data = null,
  } = parsePathname(pathname);

  if (!country) {
    throw new TypeError('country must be provided in the permalink');
  }

  if (!lang) {
    throw new TypeError('lang must be provided in the permalink');
  }

  return {
    href,
    origin,
    pathname,
    country,
    lang,
    contentType,
    title,
    id,
    data,
  };
};

export function normalize({ country, contentType, id }) {
  const components = [
    'https://www.karrotmarket.com',
    country,
    contentType,
    id,
  ];

  if (!components.every(Boolean)) {
    throw new TypeError('Invalid permalink');
  }

  return ensureTrailingSlash(components.join('/'));
};

export function canonicalize(permalink, title) {
  if (title == null) {
    throw new TypeError('You muse bind the title property explicitly, try again with canonicalize(permalink, permalink.title)');
  }

  const { country, lang, contentType, id } = permalink;
  const origin = wellKnownCountryToOrigin[country];
  const canonicalOrigin = aliases[origin] || origin;

  const components = [
    canonicalOrigin,
    lang || wellKnownCountryToLanguage[country],
    contentType,
    encodeURIComponent(slugify(`${title || ''}-${id}`)),
  ];

  if (!components.every(Boolean)) {
    throw new TypeError('Invalid permalink');
  }

  return ensureTrailingSlash(components.join('/'));
};
