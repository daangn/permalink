# Web Content Permalink Specification

Version: 2021-08-06
Status: Draft

## Introduction

당근마켓 컨텐츠를 웹에 노출 할 때 사용하는 퍼머링크 표준입니다.

일반적으로 HTTP([RFC 2616], [RFC 7540])를 사용한 통신 상황임을 가정합니다.

문서에서 사용된 MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL 키워드는 [RFC 2119]의 정의를 따릅니다.

## ABNF

```abnf
pathname      = ["/" country]                      ; Optional if base URL is well-known
                ["/" lang]                         ; Optional if base country is well-known
                "/" content-type
                "/" slug
                ["/" data]                         ; Alternative to query string (client-only)
                "/"                                ; Should be ended with a trailing slash
                
country       = 2UPALPHA                           ; A - Z, ISO 3166-1 (alpha 2) code
lang          = 2LOALPHA                           ; a - z, ISO 639-1 code

content-type  = 2*(LOALPHA / "-")
slug          = [title "-"] id                     ; SEO-friendly identifier for the content
title         = 1*uriencoded *("-" (1*uriencoded)) ; content title
id            = 8*(alphanum / "_")                 ; content identifier
data          = 1*alphanum                         ; lz-based compressed data string for client-only
uriencoded    = (alphanum / pct-encoded)           ; pct-encoded refers to rfc3986
alphanum      = (ALPHA / DIGIT)                    ; letters and numbers
```

`pct-encoded`는 [RFC 3986]의 정의를 따릅니다.

## Slug-only

퍼머링크는 전체 URL이 아닌 경로이름을 사용해서 정의합니다. (MUST)

따라서 다음 링크들은 동일한 퍼머링크로 취급됩니다.

- `http://localhost:8080/KR/ko/my-article-1234/?var=foo`
- `https://www.daangn.com/KR/ko/my-article-1234/?var=bar`
- `https://karrot.app/KR/ko/my-article-1234/#hash`

## Training Slash

퍼머링크는 마지막에 슬래시 문자(/)로 종료되어야 합니다 (RECOMMENDED)

- `https://www.daangn.com/test-1234/?var=foo` -> Valid
- `https://www.daangn.com/test-1234?bar=foo` -> Invalid

경로를 해석할 때 가능한 슬래시 문자로 종료되도록 정규화 합니다.

사용자가 슬래시로 끝나지 않는 경로로 요청하는 경우 슬래시 경로로 리다이렉션(HTTP 302) 합니다. 정적 파일을 기반 사이트 운영 시 웹 서버와 브라우저의 컨텐츠 협상 알고리듬에 의해 자동으로 달성됩니다.

## Properties

### Country

`country` 속성은 [ISO 3166-1] (alpha-2)로 정의합니다.

### Language

`lang` 속성은 [ISO-639-1]로 정의합니다.

### Content Type

`content-type` 속성은 서비스에 대한 고유 식별자입니다.

### Content Identifier

`id` 속성은 컨텐츠 퍼머링크의 고유성을 부여하는 목적으로만 사용합니다.

다른 구성 맥락(서비스, DB, 클러스터, 기타 인프라 등)과 관계없이 월드와이드웹에 노출되는 것을 기점으로 영구적입니다. (RECOMMENDED)

DB 레코드의 고유성과 연관짓지 않아야 합니다. (RECOMMENDED)

물리적인 시간 외의 다른 유추가능한 정보와 연관짓지 않아야 합니다. (RECOMMENDED)

### Content Title

SEO 목적을 위해 컨텐츠의 제목 속성이 퍼머링크에 포함될 수 있습니다.

<details>
  <summary>설명</summary>
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

#### `title` 속성이 변경되는 경우?

컨텐츠 제목 변경이 컨텐츠 식별에 영향을 주지 않아야 합니다 (MUST)

<details>
  <summary>다른 서비스 퍼머링크 예시</summary>
  <div>
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
  </div>
</details>

### Link Data

클라이언트에 전달할 목적으로 압축된 추가 데이터를 `data` 속성에 포함합니다 (OPTIONAL)

#### Data Codec

데이터는 압축된 JSON 메시지(또는 그와 동등한 표현을 가진 객체)입니다.

코덱으로 [MessagePack] + Base58를 사용합니다 (SHOULD)

- 인코딩: 주어진 JSON 데이터를 MessagePack 바이너리로 인코딩합니다. MessagePack 바이너리를 Base58로 인코딩합니다.
- 디코딩: 주어진 Base58 문자열을 바이너리로 디코딩 합니다. 출력한 바이너리를 MessagePack 코덱으로 JSON 객체로 복원합니다.

#### Base58 문자 테이블

다음 문자 테이블을 사용합니다. (SHOULD)

`123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`
(Bitcoin 프로젝트에서 사용하는 문자 테이블과 동일)

## Context & Well-knowns

퍼머링크는 여러 맥락을 포함하고 있습니다. 이 중 국가와 지역은 다른 주변 맥락에 따라 암묵적으로 해석될 수 있습니다.

### Origin aliases

주어진 URL Origin에 대해서 알려진 별칭이 있는 경우 해석할 때 별칭을 사용합니다.

- `https://karrotmarket.com` → `https://www.karrotmarket.com`
- `https://daangn.com` → `https://www.daangn.com`

### Well-known hosts

잘 알려진 (Well-known) 호스트가 사용되어 추론이 가능한 경우 `country` 속성은 생략할 수 있습니다.

나머지 경우는 `country` 속성이 퍼머링크에 포함되어야 합니다. (SHOULD)

- `https://www.karrotmarket.com` (Unknown)
- `https://www.daangn.com` (`KR`)
- `https://ca.karrotmarket.com` (`CA`)
- `https://uk.karrotmarket.com` (`UK`)
- `https://us.karrotmarket.com` (`US`)
- `https://jp.karrotmarket.com` (`JP`)

### Well-known countries

잘 알려진 (Well-known) `country` 속성이 맥락에 포함된 경우 `lang` 속성을 생략할 수 있습니다.

나머지 경우는 `lang` 속성이 퍼머링크에 포함되거나, 프로그램에서 제공되어야 합니다. (SHOULD)

- `KR` (`ko`)
- `CA` (`en`)
- `UK` (`en`)
- `US` (`en`)
- `JP` (`ja`)

## Referencing

퍼머링크를 통해 컨텐츠를 참조할 때 필요한 규칙들을 설명합니다.

### Property Stability

안정된(Stable) 속성

- `country`
- `content-type`
- `id`

불안정한(Unstable) 속성

- `lang`: 사용자의 브라우징 맥락에 의존적입니다.
- `title`: 방문자를 위해서만 제공되며 서비스에서 일관성을 보장하지 않습니다.
- `data`: 클라이언트 애플리케이션을 위해서 제공되며 서비스에서 일관성을 보장하지 않습니다.

### Normalization

컨텐츠가 다른 컨텐츠를 참조할 때 정규화된 퍼머링크를 사용합니다 (MUST)

퍼머링크를 정규화하는 절차는 다음과 같습니다.

1. 호스트를 `https://www.karrotmarket.com` 으로 변경합니다.
2. 안정된 속성을 모두 포함합니다.
3. 불안정한 속성을 모두 제거합니다.

### Uniqueness

정규화된 퍼머링크는 고유한 컨텐츠를 가르켜야 합니다. (MUST)

더 세부적인 맥락에서 고유성은 “안정된 속성”들만 사용해서 판별하게 됩니다.
동일한 `id`, `content-type`, `country` 조합이 가르키는 컨텐츠는 반드시 고유해야합니다.

세부적인 고유성 수준은 서비스에서 결정합니다.

- 사실, `id`만으로 찾는 컨텐츠를 특정할 수 있습니다.
- 사실, `id`와 `content-type`의 조합으로 컨텐츠를 특정할 수 있습니다.
- `id`, `content-type`, `country`의 조합으로 컨텐츠를 특정할 수 있습니다.

이 맥락을 직접 해석하는 것 보다 퍼머링크를 불변 참조값으로 다루는 것을 더 권장합니다.

### Canonicalization

하나의 컨텐츠가 여러 퍼머링크를 가질 수 있습니다.

- 타이틀을 포함하지 않는 경우
- 언어가 다른 경우
- 데이터가 다른 경우

여러 퍼머링크들 중 하나가 컨텐츠를 대표하는 URL(Canonical URL)로 사용됩니다.

#### 여러 퍼머링크 중 대표 URL을 선택하는 방법

- `https` 프로토콜을 사용합니다. (MUST)
- `country` 속성을 생략할 수 있는 경우, 생략된 퍼머링크가 더 높은 우선순위를 갖습니다. (“Context & Well-known URL” 섹션 참고)
- `lang` 속성이 명시된 경우 더 높은 우선순위를 갖습니다. (“Context & Well-known URL” 섹션 참고)
- `data` 속성은 대표 URL에 포함하지 않습니다. (“Link Data” 섹션 참고)
- `title` 속성은 대표 URL에 포함합니다.
- 가장 최신의 `title` 속성 값을 포함한 퍼머링크가 더 높은 우선순위를 갖습니다.

사용자가 퍼머링크로 요청 했을 때 서비스는 응답(웹 페이지 등)에 대표 URL에 대한 링크를 포함하거나 301 응답을 통해 사용자를 대표 URL로 리다이렉트 해야 합니다. (SHOULD)

301 응답은 중복된 URL이 더 이상 사용되지 않는 것을 의미합니다. 컨텐츠를 보존하는데 기술적 제약이 없는 한 리다이렉션 응답을 사용하지 않습니다 (RECOMMENDED)

<details>
  <summary>더 자세한 설명</summary>
  <a href="https://developers.google.com/search/docs/advanced/crawling/consolidate-duplicate-urls">
    Consolidate Duplicate URLs
  </a>
</details>

#### Internalization

한 컨텐츠는 다양한 언어 맥락에서 제공될 수 있습니다. 또는 컨텐츠 내용이 여러 언어로 제공되지 않더라도 컨텐츠를 포함하는 UI 요소가 여러 언어로 제공될 수 있습니다.

사용자의 언어 맥락을 보존하기 위해 외부에 공유되는 퍼머링크와 대표 URL에 `lang` 속성을 포함합니다. (SHOULD)

대표 URL은 컨텐츠가 실제로 사용하는 언어를 우선해서 선택합니다 (RECOMMENDED)

## References

[RFC 2616]: https://datatracker.ietf.org/doc/html/rfc2616
[RFC 7540]: https://datatracker.ietf.org/doc/html/rfc7540
[RFC 2119]: https://datatracker.ietf.org/doc/html/rfc2119
[RFC 3986]: https://datatracker.ietf.org/doc/html/rfc3986#section-2.1
[ISO-3166-1]: https://www.iso.org/iso-3166-country-codes.html
[ISO-639-1]: https://www.iso.org/iso-639-language-codes.html
