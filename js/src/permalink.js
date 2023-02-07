import slugify from 'cjk-slug';
import { parsePathname } from './pathname.js';

let wellKnownOriginToCountry = {
  'https://daangn.com': 'kr',
  'https://karrotmarket.com': undefined,
  'https://www.daangn.com': 'kr',
  'https://www.karrotmarket.com': undefined,
  'https://ca.karrotmarket.com': 'ca',
  'https://jp.karrotmarket.com': 'jp',
  'https://uk.karrotmarket.com': 'uk',
  'https://us.karrotmarket.com': 'us',
  'https://kr.karrotmarket.com': 'kr',
};

let wellKnownCountryToOrigin = {
  'ca': 'https://ca.karrotmarket.com',
  'jp': 'https://jp.karrotmarket.com',
  'uk': 'https://uk.karrotmarket.com',
  'us': 'https://us.karrotmarket.com',
  'kr': 'https://www.daangn.com',
};

let wellKnownCountryToLanguage = {
  'ca': 'en',
  'jp': 'ja',
  'kr': 'ko',
  'uk': 'en',
  'us': 'en',
};

let aliases = {
  'https://daangn.com': 'https://www.daangn.com',
  'https://karrotmarket.com': 'https://www.karrotmarket.com',
};

function ensureTrailingSlash(pathLike) {
  return pathLike.endsWith('/') ? pathLike : pathLike + '/';
}

export function parse(urlLike) {
  let url = new URL(urlLike);
  url.pathname = ensureTrailingSlash(url.pathname);

  let { origin, pathname } = url;

  let parseResult = parsePathname(pathname);
  if (!parseResult) {
    throw new TypeError('Invalid permalink format');
  };

  let {
    country,
    serviceType,
    title = null,
    id,
    data = null,
  } = parseResult;

  country = country.toLowerCase();
  country = wellKnownOriginToCountry[origin] || wellKnownOriginToCountry[aliases[origin]] || country;

  let defaultLanguage = wellKnownCountryToLanguage[country];
  if (!defaultLanguage) {
    throw new TypeError(`defaultLanguage cannot be inferred since the country ${country} is unknown`);
  }

  return {
    country,
    defaultLanguage,
    serviceType,
    title,
    id,
    data,
  };
};

export function normalize({ country, serviceType, id }) {
  let components = [
    'https://www.karrotmarket.com',
    country,
    serviceType,
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

  let { country, serviceType, id } = permalink;
  country = country.toLowerCase();

  let origin = wellKnownCountryToOrigin[country];
  let canonicalOrigin = aliases[origin] || origin;

  let components = [
    canonicalOrigin,
    country,
    serviceType,
    encodeURIComponent(slugify(`${title || ''}-${id}`)),
  ];

  if (!components.every(Boolean)) {
    throw new TypeError('Invalid permalink');
  }

  return ensureTrailingSlash(components.join('/'));
};
