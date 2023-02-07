# Web Content Permalink Specification

- Version: 2023-02-07
- Status: Draft

## Introduction

당근마켓 컨텐츠를 웹에 노출 할 때 사용하는 퍼머링크 표준입니다.

일반적으로 HTTP([RFC 2616], [RFC 7540])를 사용한 통신 상황임을 가정합니다.

문서에서 사용된 MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL 키워드는 [RFC 2119]의 정의를 따릅니다.

## Definitions

- 웹 컨텐츠: 서비스에서 월드와이드웹으로 배포한 고유 엔티티로 사용자에게 유용한 정보를 포함하며 사이트의 구성과 상관 없이 영구적으로 보존됩니다.
- 웹 컨텐츠 퍼머링크: 웹 컨텐츠를 가르키는 고유 URL 입니다.

## Pathname

퍼머링크의 형식은 전체 URL이 아닌 경로이름을 사용해서 정의합니다. (MUST)

### Format

```abnf
pathname      = country
                "/" service-type
                "/" slug
                ["/" data]                         ; Alternative to query string (client-only)
                "/"                                ; Should be ended with a trailing slash
                
country       = 2ALPHA                             ; ISO 3166-1 (alpha 2) code (case-insensitive)

service-type  = 3*(LOALPHA / "-")

slug          = [title "-"] id                     ; SEO-friendly identifier for the content
title         = 1*uriencoded *("-" (1*uriencoded)) ; content title
id            = 8*(alphanum / "_")                 ; content identifier

data          = 1*alphanum                         ; lz-based compressed data string for client-only

uriencoded    = alphanum / pct-encoded
pct-encoded   = "%" HEXDIG HEXDIG                  ; pct-encoded refers to rfc3986
alphanum      = ALPHA / DIGIT                      ; letters and numbers
```

주어진 속성이 모두 일치하는 경우 같은 퍼머링크로 취급됩니다. (MUST) 예를 들면 아래 퍼머링크들은 모두 같은 것으로 취급합니다.

- `http://localhost:8080/kr/my-article-1234/?var=foo`
- `https://www.daangn.com/kr/my-article-1234/?var=bar`
- `https://karrot.app/kr/my-article-1234/#hash`

#### Country

`country` 속성은 [ISO 3166-1] (alpha-2)로 정의합니다.

반드시 경로에 포함되어야 합니다. (MUST)

#### Default Language

`defaultLanguage` 속성은 [ISO 639-1]로 정의합니다.

경로에 값이 포함되지 않고 항상 `country` 로 부터 추론됩니다. (“Well-known countries” 섹션 참고)

#### Service Type

`service-type` 속성은 서비스에 대한 고유 식별자입니다.

반드시 경로에 포함되어야 합니다. (MUST)

#### Content Identifier

`id` 속성은 컨텐츠 퍼머링크의 고유성을 부여하는 목적으로 사용합니다.

컨텐츠가 월드와이드웹에 노출되는 것을 기점으로 영구적으로 보존합니다. (SHOULD)

다른 구성 맥락(서비스, DB, 클러스터, 기타 인프라 등)과 연관짓지 않아야 합니다. (SHOULD) 구성 맥락이 변경되는 경우, 컨텐츠 보존이 어려워질 수 있습니다.

유추가능한 정보나 특정 알고리듬과에 의존하지 않아야 합니다. (SHOULD) ID를 식별하는데 특정 알고리듬을 의존하는 경우, 구현 변경에 의해 컨텐츠 보존이 어려워질 수 있습니다.

#### Content Title

SEO 목적을 위해 컨텐츠의 제목 속성이 퍼머링크에 포함될 수 있습니다.

<details>
  <summary>참조</summary>
  <div>
    URL에서 적절한 키워드 사용은 검색 결과에 큰 영향을 미치며, 링크 미리보기를 확인하는 사용자에게 컨텐츠에 대한 신뢰를 더해주는 효과가 있습니다.
    <ul>
      <li>
        <a href="https://www.stephanspencer.com/matt-cutts-interview/">
          Interview with Google’s Matt Cutts at Pubcon
        </a>
      </li>
      <li>
        <a href="https://youtu.be/gRzMhlFZz9I">
          Does the position of keywords in the URL affect ranking?
        </a>
      </li>
      <li>
        <a href="https://youtu.be/971qGsTPs8M">
          Is it better to have keywords in the URL path or filename?
        </a>
      </li>
    </ul>
  </div>
</details>

##### `title` 속성이 변경되는 경우?

컨텐츠 제목 변경이 컨텐츠 식별에 영향을 주지 않아야 합니다 (MUST)

<details>
  <summary>다른 서비스 퍼머링크 예시</summary>
  <ul>
    <li>
      <code>https://apps.apple.com/kr/app/당근마켓-대한민국-1등-동네-커뮤니티/id1018769995</code>
      <p>제목 부분을 제외해도 같은 페이지로 연결, Canonical URL에는 항상 최신 제목을 포함</p>
    </li>
    <li>
      <code>https://medium.com/daangn/당근마켓-ai의-데이터-활용-방법-e5aeac08bc57</code>
      <p>제목 부분을 제외해도 같은 페이지로 연결, 최신 제목 포함한 주소로 사용자 리디렉션</p>
    </li>
  </ul>
</details>

#### Link Data

클라이언트에 전달할 목적으로 압축된 추가 데이터를 `data` 속성에 포함합니다 (OPTIONAL)

##### Data Codec

데이터는 문자열 형식으로 압축된 JSON 메시지(또는 그와 동등한 표현을 가진 객체)입니다. 이 때 사용된 직렬화 형식은 반드시 URL-safe 해야 합니다. (MUST)

진입점에 `data` 속성이 주어지는 경우 반드시 클라이언트까지 유실없이 전달되어야 합니다. (MUST)

하지만 세부적인 내용이나 형식은 보장되지 않기 때문에 반드시 있을 것을 전제하지 않고 옵셔널한 값으로 취급해야 합니다. (MUST)

### Trailing Slash

경로는 슬래시 문자(`/`)로 종료되어야 합니다 (RECOMMENDED) 경로를 해석할 때 항상 슬래시 문자로 종료되도록 정규화 합니다.

예시: `https://www.daangn.com/test-1234/?var=foo` -> `https://www.daangn.com/test-1234?bar=foo`


<details>
  <summary>참조</summary>
  <ul>
    <li>
      <a href="https://developers.google.com/search/blog/2010/04/to-slash-or-not-to-slash">
        To slash or not to slash
      </a>
    </li>
    <li>
      <a href="https://cdivilly.wordpress.com/2014/03/11/why-trailing-slashes-on-uris-are-important/">
        Why trailing slashes on URIs are important
      </a>
    </li>
    <li>
      <a href="https://ahrefs.com/blog/trailing-slash/">
        Should You Have a Trailing Slash at the End of URLs?
      </a>
    </li>
  </ul>
</details>

## Context & Well-knowns

퍼머링크에 포함된 여러 속성은 정적으로 해석될 수 있습니다.

이 중 국가와 지역은 다른 주변 맥락에 따라 암묵적으로 해석될 수 있습니다.

### Origin aliases

주어진 URL Origin에 대해서 알려진 별칭이 있는 경우 해석할 때 별칭을 사용합니다.

- `https://karrotmarket.com` → `https://www.karrotmarket.com`
- `https://daangn.com` → `https://www.daangn.com`

### Well-known hosts

`country` 속성 값은 항상 퍼머링크에 포함됩니다. 하지만 주어진 호스트가 잘 알려진 호스트인 경우 주어진 `country` 속성 값보다 알려진 값을 우선합니다.

- `https://www.daangn.com` (`KR`)
- `https://kr.karrotmarket.com` (`KR`)
- `https://ca.karrotmarket.com` (`CA`)
- `https://uk.karrotmarket.com` (`UK`)
- `https://us.karrotmarket.com` (`US`)
- `https://jp.karrotmarket.com` (`JP`)

### Well-known countries

`country` 속성에 따라 `defaultLanguage` 속성이 기본 제공됩니다.

- `KR` (`ko`)
- `CA` (`en`)
- `UK` (`en`)
- `US` (`en`)
- `JP` (`ja`)

## Referencing

퍼머링크를 통해 컨텐츠를 참조할 때 필요한 규칙들을 설명합니다.

### Property Stability

안정적인(Stable) 속성

- `country`
- `service-type`
- `id`

불안정한(Unstable) 속성

- `title`: 방문자를 위해서만 제공되며 서비스에서 일관성을 보장하지 않습니다.
- `data`: 클라이언트 애플리케이션을 위해서 제공되며 서비스에서 일관성을 보장하지 않습니다.

### Normalization

컨텐츠가 다른 컨텐츠를 참조할 때 정규화된 퍼머링크를 사용합니다 (MUST)

퍼머링크를 정규화하는 절차는 다음과 같습니다.

1. 호스트를 `https://www.karrotmarket.com` 으로 변경합니다.
2. 안정적인 속성을 모두 포함합니다.
3. 불안정한 속성을 모두 제거합니다.

### Uniqueness

정규화된 퍼머링크는 영구적으로 고유한 컨텐츠를 가르켜야 합니다. (MUST)

더 세부적인 맥락에서 고유성은 “안정적인 속성”들만 사용해서 판별하게 됩니다.
동일한 안정적인 속성(`id`, `content-type`, `country`) 조합이 가르키는 컨텐츠는 반드시 고유해야합니다.

구체적인 고유성 수준은 서비스에서 결정합니다.

- `id`만으로 찾는 컨텐츠를 특정할 수 있는 경우.
- `id`와 `service-type`의 조합으로 컨텐츠를 특정할 수 있는 경우.
- `id`, `service-type`, `country`의 조합으로 컨텐츠를 특정할 수 있는 경우.

속성을 직접 해석하는 것 보다 퍼머링크 자체를 불변 참조값으로 다루는 것을 권장합니다.

### Canonicalization

불안정한 속성들로 인해 하나의 컨텐츠가 여러 퍼머링크를 가질 수 있습니다.

여러 퍼머링크들 중 하나가 컨텐츠를 대표하는 URL(Canonical URL)로 사용됩니다.

#### 여러 퍼머링크 중 대표 URL을 선택하는 방법

- `https` 프로토콜을 사용합니다.
- 잘 알려진 호스트를 사용합니다. (“Well-known hosts” 섹션 참고)
- `data` 속성을 대표 URL에 포함하지 않습니다. (“Link Data” 섹션 참고)
- `title` 속성을 대표 URL에 포함합니다.
- 가장 최신의 `title` 속성 값을 포함한 퍼머링크가 더 높은 우선순위를 갖습니다.

사용자가 퍼머링크로 요청 했을 때 서비스는 응답(웹 페이지 등)에 대표 URL에 대한 링크를 포함하거나 HTTP 301 응답을 통해 사용자를 대표 URL로 리다이렉트 해야 합니다. (SHOULD)

HTTP 301 응답은 중복된 URL이 더 이상 사용되지 않는 것을 의미합니다. 의미가 동일하지 않다면 컨텐츠를 보존하는데 기술적 제약이 없는 한 리다이렉션 응답을 사용하지 않습니다 (RECOMMENDED)

<details>
  <summary>참조</summary>
  <ul>
    <li>
      <a href="https://developers.google.com/search/docs/advanced/crawling/consolidate-duplicate-urls">
        Consolidate Duplicate URLs
      </a>
    </li>
  </ul>
</details>

#### Internalization

한 컨텐츠는 다양한 언어 맥락에서 제공될 수 있습니다. 또는 컨텐츠 내용이 여러 언어로 제공되지 않더라도 컨텐츠를 포함하는 UI 요소가 여러 언어로 제공될 수 있습니다.

다만 당근마켓은 지역 중심의 커뮤니티 서비스로, 당근마켓에서 제공하는 웹 컨텐츠는 항상 지역 중심적인 맥락을 가지고 있습니다. 그에 따라 컨텐츠의 내용도 해당 지역에서 선호되는 언어로 작성되어 있을 가능성이 높습니다.

당근마켓의 퍼머링크 표준은 컨텐츠를 위한 것으로 언어 맥락을 의도적으로 포함하지 않으며, 컨텐츠와 무관한 UI 설정 등은 앱이나 브라우저 등 사용자 에이전트의 설정을 따릅니다. 웹 컨텐츠의 언어는 이 설정에 영향을 받지 않습니다.

## References

- [Interview with Google’s Matt Cutts at Pubcon](https://www.stephanspencer.com/matt-cutts-interview/)
- [Does the position of keywords in the URL affect ranking?](https://youtu.be/gRzMhlFZz9I)
- [Is it better to have keywords in the URL path or filename?](https://youtu.be/971qGsTPs8M)
- [To slash or not to slash](https://developers.google.com/search/blog/2010/04/to-slash-or-not-to-slash)
- [Why trailing slashes on URIs are important](https://cdivilly.wordpress.com/2014/03/11/why-trailing-slashes-on-uris-are-important/)
- [Should You Have a Trailing Slash at the End of URLs?](https://ahrefs.com/blog/trailing-slash/)
- [Consolidate Duplicate URLs](https://developers.google.com/search/docs/advanced/crawling/consolidate-duplicate-urls)

[RFC 2616]: https://datatracker.ietf.org/doc/html/rfc2616
[RFC 7540]: https://datatracker.ietf.org/doc/html/rfc7540
[RFC 2119]: https://datatracker.ietf.org/doc/html/rfc2119
[RFC 3986]: https://datatracker.ietf.org/doc/html/rfc3986#section-2.1
[ISO 3166-1]: https://www.iso.org/iso-3166-country-codes.html
[ISO 639-1]: https://www.iso.org/iso-639-language-codes.html
