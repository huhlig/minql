//
// Copyright 2024 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::{
    Authority, Fragment, HostInfo, Path, Query, Scheme, URIError, URIReference,
    URIRelativeReference, URIResult, UserInfo, URI,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{char as nchar, digit1, one_of},
    combinator::{consumed, map, not, opt, peek, recognize},
    error::{ErrorKind, ParseError},
    multi::{many0, many1, many_m_n, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

impl<'str> URI<'str> {
    /// Parse a string into a Uniform Resource Identifier
    #[tracing::instrument(level = "trace")]
    pub fn parse(input: &'str str) -> URIResult<URI<'str>> {
        match uri::<(&str, ErrorKind)>(input) {
            Ok((_, url)) => Ok(url),
            Err(err) => Err(URIError::Parsing(err.to_string())),
        }
    }
}

impl<'str> URIReference<'str> {
    /// Parse a string into a Uniform Resource Identifier Reference
    #[tracing::instrument(level = "trace")]
    pub fn parse(input: &'str str) -> URIResult<URIReference<'str>> {
        match uri_reference::<(&str, ErrorKind)>(input) {
            Ok((_, url)) => Ok(url),
            Err(err) => Err(URIError::Parsing(err.to_string())),
        }
    }
}

impl<'str> URIRelativeReference<'str> {
    /// Parse a string into a Uniform Resource Identifier Relative Reference
    #[tracing::instrument(level = "trace")]
    pub fn parse(input: &'str str) -> URIResult<URIRelativeReference<'str>> {
        match relative_ref::<(&str, ErrorKind)>(input) {
            Ok((_, rel_ref)) => Ok(rel_ref),
            Err(err) => Err(URIError::Parsing(err.to_string())),
        }
    }
}

impl<'str> Path<'str> {
    /// Parse a string into a Uniform Resource Identifier Path
    #[tracing::instrument(level = "trace")]
    pub fn parse(input: &'str str) -> URIResult<Path<'str>> {
        match path::<(&str, ErrorKind)>(input) {
            Ok((_, path)) => Ok(path),
            Err(err) => Err(URIError::Parsing(err.to_string())),
        }
    }
}

///
/// ```abnf
/// URI           = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
/// absolute-URI  = scheme ":" hier-part [ "?" query ]
/// ```
/// * Absolute URI doesn't matter for parsing as fragment is optional
#[tracing::instrument(level = "trace")]
fn uri<'str, E>(input: &'str str) -> IResult<&'str str, URI<'str>, E>
where
    E: ParseError<&'str str>,
{
    map(
        consumed(tuple((
            terminated(scheme, nchar(':')),
            hier_part,
            opt(preceded(nchar('?'), query)),
            opt(preceded(nchar('#'), fragment)),
        ))),
        |(string, (scheme, (authority, path), query, fragment))| URI {
            string,
            scheme,
            authority,
            path,
            query,
            fragment,
        },
    )(input)
}

/// ```abnf
/// hier-part     = "//" authority path-abempty
///               / path-absolute
///               / path-rootless
///               / path-empty
/// ```
#[tracing::instrument(level = "trace")]
fn hier_part<'str, E>(
    input: &'str str,
) -> IResult<&'str str, (Option<Authority<'str>>, Path<'str>), E>
where
    E: ParseError<&'str str>,
{
    alt((
        map(
            preceded(tag("//"), pair(authority, path_abempty)),
            |(authority, path)| (Some(authority), path),
        ),
        map(path_absolute, |path| (None, path)),
        map(path_rootless, |path| (None, path)),
        map(path_empty, |path| (None, path)),
    ))(input)
}

/// ```abnf
/// URI-reference = URI / relative-ref
/// ```
#[tracing::instrument(level = "trace")]
fn uri_reference<'str, E>(input: &'str str) -> IResult<&'str str, URIReference<'str>, E>
where
    E: ParseError<&'str str>,
{
    alt((
        map(uri, |uri| URIReference::Absolute(uri)),
        map(relative_ref, |uri_ref| URIReference::Relative(uri_ref)),
    ))(input)
}

/// ```abnf
/// relative-ref  = relative-part [ "?" query ] [ "#" fragment ]
/// ```
#[tracing::instrument(level = "trace")]
fn relative_ref<'str, E>(input: &'str str) -> IResult<&'str str, URIRelativeReference<'str>, E>
where
    E: ParseError<&'str str>,
{
    map(
        consumed(tuple((
            relative_part,
            opt(preceded(nchar('?'), query)),
            opt(preceded(nchar('#'), fragment)),
        ))),
        |(string, ((authority, path), query, fragment))| URIRelativeReference {
            string,
            authority,
            path,
            query,
            fragment,
        },
    )(input)
}

/// ```abnf
/// relative-part = "//" authority path-abempty
///               / path-absolute
///               / path-noscheme
///               / path-empty
/// ```
#[tracing::instrument(level = "trace")]
fn relative_part<'str, E>(
    input: &'str str,
) -> IResult<&'str str, (Option<Authority<'str>>, Path<'str>), E>
where
    E: ParseError<&'str str>,
{
    alt((
        map(
            preceded(tag("//"), pair(authority, path_abempty)),
            |(authority, path)| (Some(authority), path),
        ),
        map(path_absolute, |path| (None, path)),
        map(path_noscheme, |path| (None, path)),
        map(path_empty, |path| (None, path)),
    ))(input)
}

/// ```abnf
/// scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
/// ```
#[tracing::instrument(level = "trace")]
fn scheme<'str, E>(input: &'str str) -> IResult<&'str str, Scheme<'str>, E>
where
    E: ParseError<&'str str>,
{
    alt((
        map(tag_no_case("HTTPS"), |_| Scheme::HTTPS),
        map(tag_no_case("HTTP"), |_| Scheme::HTTP),
        map(
            recognize(pair(alpha, many0(alt((alpha, digit, one_of("+-.")))))),
            |str| Scheme::Other(str),
        ),
    ))(input)
}

/// ```abnf
/// authority     = [ userinfo "@" ] host [ ":" port ]
/// ```
#[tracing::instrument(level = "trace")]
fn authority<'str, E>(input: &'str str) -> IResult<&'str str, Authority<'str>, E>
where
    E: ParseError<&'str str>,
{
    map(
        consumed(tuple((
            opt(terminated(userinfo, nchar('@'))),
            host,
            opt(preceded(nchar(':'), port)),
        ))),
        |(string, (userinfo, hostinfo, port))| Authority {
            string,
            userinfo,
            hostinfo,
            port,
        },
    )(input)
}

/// ```abnf
/// userinfo      = *( unreserved / pct-encoded / sub-delims / ":" )
/// ```
/// Secondary Parsing
/// ```abnf
/// userinfo      = username [ ":" password ]
/// username      = 1*( unreserved / pct-encoded / sub-delims )
/// password      = 1*( unreserved / pct-encoded / sub-delims / ":" )
/// ```
#[tracing::instrument(level = "trace")]
fn userinfo<'str, E>(input: &'str str) -> IResult<&'str str, UserInfo<'str>, E>
where
    E: ParseError<&'str str>,
{
    let username = recognize(many1(alt((unreserved, pct_encoded, sub_delims))));
    let password = recognize(many1(alt((
        unreserved,
        pct_encoded,
        sub_delims,
        nchar(':'),
    ))));
    let (input, string) = recognize(many1(alt((
        unreserved,
        pct_encoded,
        sub_delims,
        nchar(':'),
    ))))(input)?;
    let res: IResult<&str, (&str, Option<&str>), E> =
        pair(username, opt(map(pair(nchar(':'), password), |(_, a)| a)))(string);

    if let Ok((_, (username, password))) = res {
        Ok((
            input,
            UserInfo::Parsed {
                string,
                username,
                password,
            },
        ))
    } else {
        Ok((input, UserInfo::Unparsed(string)))
    }
}

/// ```abnf
/// host          = IP-literal / IPv4address / reg-name
/// IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"
/// ```
#[tracing::instrument(level = "trace")]
fn host<'str, E>(input: &'str str) -> IResult<&'str str, HostInfo<'str>, E>
where
    E: ParseError<&'str str>,
{
    // TODO: Fix Weird Parsing
    alt((
        map(delimited(nchar('['), ip_v6_address, nchar(']')), |string| {
            HostInfo::IPv6Address {
                string,
                ipaddr: Ipv6Addr::from_str(string).unwrap(),
            }
        }),
        map(delimited(nchar('['), ip_v_future, nchar(']')), |string| {
            HostInfo::IPvFutureAddress { string }
        }),
        map(ip_v4_address, |string| HostInfo::IPv4Address {
            string,
            ipaddr: Ipv4Addr::from_str(string).unwrap(),
        }),
        map(reg_name, |string| HostInfo::RegistryName { string }),
    ))(input)
}

/// ```abnf
/// port          = *DIGIT
/// ```
#[tracing::instrument(level = "trace")]
fn port<'str, E>(input: &'str str) -> IResult<&'str str, u16, E>
where
    E: ParseError<&'str str>,
{
    let (input, str) = digit1(input)?;
    let val = u16::from_str_radix(str, 10)
        .map_err(|_| nom::Err::Error(E::from_error_kind(input, ErrorKind::HexDigit)))?;
    Ok((input, val))
}

/// ```abnf
/// IPvFuture     = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
/// ```
#[tracing::instrument(level = "trace")]
fn ip_v_future<'str, E>(input: &'str str) -> IResult<&'str str, &'str str, E>
where
    E: ParseError<&'str str>,
{
    recognize(tuple((
        nchar('v'),
        many1(hexdig),
        nchar('.'),
        many1(alt((unreserved, sub_delims, nchar(':')))),
    )))(input)
}

/// ```abnf
/// IPv6address   =                            6( h16 ":" ) ls32
///               /                       "::" 5( h16 ":" ) ls32
///               / [               h16 ] "::" 4( h16 ":" ) ls32
///               / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
///               / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
///               / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
///               / [ *4( h16 ":" ) h16 ] "::"              ls32
///               / [ *5( h16 ":" ) h16 ] "::"              h16
///               / [ *6( h16 ":" ) h16 ] "::"
/// ```
#[rustfmt::skip]
#[tracing::instrument(level = "trace")]
fn ip_v6_address<'str, E>(input: &'str str) -> IResult<&'str str, &'str str, E>
where
    E: ParseError<&'str str>,
{

    alt((
        //                            6( h16 ":" ) ls32
        recognize(tuple((                                                                                    many_m_n(6, 6, pair(h16, nchar(':'))), ls32))),
        //                       "::" 5( h16 ":" ) ls32
        recognize(tuple((                                                                         tag("::"), many_m_n(5, 5, pair(h16, nchar(':'))), ls32))),
        // [               h16 ] "::" 4( h16 ":" ) ls32
        recognize(tuple((opt(                                                             h16  ), tag("::"), many_m_n(4, 4, pair(h16, nchar(':'))), ls32))),
        // [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
        recognize(tuple((opt(tuple((many_m_n(0, 1, pair(h16, nchar(':'))), h16))), tag("::"), many_m_n(3, 3, pair(h16, nchar(':'))), ls32))),
        // [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
        recognize(tuple((opt(tuple((many_m_n(0, 2, pair(h16, nchar(':'))), h16))), tag("::"), many_m_n(2, 2, pair(h16, nchar(':'))), ls32))),
        // [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
        recognize(tuple((opt(tuple((many_m_n(0, 3, pair(h16, nchar(':'))), h16))), tag("::"), many_m_n(1, 1, pair(h16, nchar(':'))), ls32))),
        // [ *4( h16 ":" ) h16 ] "::"              ls32
        recognize(tuple((opt(tuple((many_m_n(0, 4, pair(h16, nchar(':'))), h16))), tag("::"),                                                     ls32))),
        // [ *5( h16 ":" ) h16 ] "::"              h16
        recognize(tuple((opt(tuple((many_m_n(0, 5, pair(h16, nchar(':'))), h16))), tag("::"),                                                      h16))),
        // [ *6( h16 ":" ) h16 ] "::"
        recognize(tuple((opt(tuple((many_m_n(0, 6, pair(h16, nchar(':'))), h16))), tag("::")                                                          ))),
    ))(input)
}

/// ```abnf
/// h16           = 1*4HEXDIG
/// ```
#[tracing::instrument(level = "trace")]
fn h16<'str, E>(input: &'str str) -> IResult<&'str str, &'str str, E>
where
    E: ParseError<&'str str>,
{
    recognize(tuple((hexdig, hexdig, hexdig, hexdig)))(input)
}

/// ```abnf
/// ls32          = ( h16 ":" h16 ) / IPv4address
/// ```
#[tracing::instrument(level = "trace")]
fn ls32<'str, E>(input: &'str str) -> IResult<&'str str, &'str str, E>
where
    E: ParseError<&'str str>,
{
    alt((
        recognize(separated_pair(h16, nchar(':'), h16)),
        ip_v4_address,
    ))(input)
}

/// ```abnf
/// IPv4address   = dec-octet "." dec-octet "." dec-octet "." dec-octet
/// ```
#[tracing::instrument(level = "trace")]
fn ip_v4_address<'str, E>(input: &'str str) -> IResult<&'str str, &'str str, E>
where
    E: ParseError<&'str str>,
{
    recognize(tuple((
        dec_octet,
        nchar('.'),
        dec_octet,
        nchar('.'),
        dec_octet,
        nchar('.'),
        dec_octet,
    )))(input)
}

/// ```abnf
/// dec-octet     = DIGIT                 ; 0-9
///               / %x31-39 DIGIT         ; 10-99
///               / "1" 2DIGIT            ; 100-199
///               / "2" %x30-34 DIGIT     ; 200-249
///               / "25" %x30-35          ; 250-255
/// ```
#[tracing::instrument(level = "trace")]
fn dec_octet<'str, E>(input: &'str str) -> IResult<&'str str, u8, E>
where
    E: ParseError<&'str str>,
{
    let (input, str) = digit1(input)?;
    let val = u8::from_str_radix(str, 10)
        .map_err(|_| nom::Err::Error(E::from_error_kind(input, ErrorKind::Digit)))?;
    Ok((input, val))
}

/// ```abnf
/// reg-name      = *( unreserved / pct-encoded / sub-delims )
/// ```
#[tracing::instrument(level = "trace")]
fn reg_name<'str, E>(input: &'str str) -> IResult<&'str str, &'str str, E>
where
    E: ParseError<&'str str>,
{
    recognize(many0(alt((unreserved, pct_encoded, sub_delims))))(input)
}

/// ```abnf
/// path          = path-abempty    ; begins with "/" or is empty
///               / path-absolute   ; begins with "/" but not "//"
///               / path-noscheme   ; begins with a non-colon segment
///               / path-rootless   ; begins with a segment
///               / path-empty      ; zero characters
/// ```
#[allow(unused)]
#[tracing::instrument(level = "trace")]
fn path<'str, E>(input: &'str str) -> IResult<&'str str, Path<'str>, E>
where
    E: ParseError<&'str str>,
{
    alt((
        path_absolute,
        path_noscheme,
        path_rootless,
        path_abempty,
        path_empty,
    ))(input)
}

/// ```abnf
/// path-absolute = "/" [ segment-nz *( "/" segment ) ]
/// ```
#[tracing::instrument(level = "trace")]
fn path_absolute<'str, E>(input: &'str str) -> IResult<&'str str, Path<'str>, E>
where
    E: ParseError<&'str str>,
{
    let (input, (string, (seg_nz, segs))) = consumed(preceded(
        nchar('/'),
        pair(segment_nz, many0(preceded(nchar('/'), segment))),
    ))(input)?;
    let mut segments = Vec::with_capacity(1 + segs.len());
    segments.push(seg_nz);
    segments.extend(segs);
    Ok((input, Path::Absolute { string, segments }))
}

/// ```abnf
/// path-noscheme = segment-nz-nc *( "/" segment )
/// ```
#[tracing::instrument(level = "trace")]
fn path_noscheme<'str, E>(input: &'str str) -> IResult<&'str str, Path<'str>, E>
where
    E: ParseError<&'str str>,
{
    let (input, (string, (seg_nz, segs))) =
        consumed(pair(segment_nz_nc, many0(preceded(nchar('/'), segment))))(input)?;
    let mut segments = Vec::with_capacity(1 + segs.len());
    segments.push(seg_nz);
    segments.extend(segs);
    Ok((input, Path::NoScheme { string, segments }))
}

/// ```abnf
/// path-rootless = segment-nz *( "/" segment )
/// ```
#[tracing::instrument(level = "trace")]
fn path_rootless<'str, E>(input: &'str str) -> IResult<&'str str, Path<'str>, E>
where
    E: ParseError<&'str str>,
{
    let (input, (string, (seg_nz, segs))) =
        consumed(pair(segment_nz, many0(preceded(nchar('/'), segment))))(input)?;
    let mut segments = Vec::with_capacity(1 + segs.len());
    segments.push(seg_nz);
    segments.extend(segs);
    Ok((input, Path::Rootless { string, segments }))
}

/// ```abnf
/// path-abempty  = *( "/" segment )
/// ```
#[tracing::instrument(level = "trace")]
fn path_abempty<'str, E>(input: &'str str) -> IResult<&'str str, Path<'str>, E>
where
    E: ParseError<&'str str>,
{
    let (input, (string, segments)) = consumed(many0(preceded(nchar('/'), segment)))(input)?;
    Ok((input, Path::AbEmpty { string, segments }))
}

/// ```abnf
/// path-empty    = 0<pchar>
/// ```
#[tracing::instrument(level = "trace")]
fn path_empty<'str, E>(input: &'str str) -> IResult<&'str str, Path<'str>, E>
where
    E: ParseError<&'str str>,
{
    not(peek(pchar))(input)?;
    Ok((input, Path::Empty))
}

/// ```abnf
/// segment       = *pchar
/// ```
#[tracing::instrument(level = "trace")]
fn segment<'str, E>(input: &'str str) -> IResult<&'str str, &str, E>
where
    E: ParseError<&'str str>,
{
    recognize(many0(pchar))(input)
}
/// ```abnf
/// segment-nz    = 1*pchar
/// ```
#[tracing::instrument(level = "trace")]
fn segment_nz<'str, E>(input: &'str str) -> IResult<&'str str, &str, E>
where
    E: ParseError<&'str str>,
{
    recognize(many1(pchar))(input)
}

/// ```abnf
/// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
/// non-zero-length segment without any colon ":"
/// ```
#[tracing::instrument(level = "trace")]
fn segment_nz_nc<'str, E>(input: &'str str) -> IResult<&'str str, &str, E>
where
    E: ParseError<&'str str>,
{
    recognize(many1(alt((
        unreserved,
        pct_encoded,
        sub_delims,
        nchar('@'),
    ))))(input)
}

/// ```abnf
/// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
/// ```
#[tracing::instrument(level = "trace")]
fn pchar<'str, E>(i: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    alt((unreserved, pct_encoded, sub_delims, one_of(":@")))(i)
}

/// ```abnf
/// query         = *( pchar / "/" / "?" )
/// ```
///
/// Secondary Parsing:
/// ```abnf
/// query_params  = query_pair *( ( ";" / "&" ) query_pair )
/// query_pair    = *query_char ( "=" *query_char ( "," *query_char ) )
/// query_char    = 1*( unreserved / pct-encoded / "!" / "$" / "'"
///               / "(" / ")" / "*" / "+" / ":" / "@" / "/" / "?" )
/// ```
#[tracing::instrument(level = "trace")]
fn query<'str, E>(input: &'str str) -> IResult<&'str str, Query<'str>, E>
where
    E: ParseError<&'str str>,
{
    let (input, query_string) = recognize(alt((pchar, one_of("/?"))))(input)?;
    let (_, query_pairs) = separated_list0(
        one_of("&;"),
        separated_pair(
            recognize(many1(query_char)),
            nchar('='),
            separated_list0(nchar(','), recognize(many0(query_char))),
        ),
    )(query_string)?;
    Ok((
        input,
        Query {
            string: query_string,
            parameters: query_pairs,
        },
    ))
}

#[tracing::instrument(level = "trace")]
fn query_char<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    alt((unreserved, pct_encoded, one_of("!$'()*+:@/?")))(input)
}

/// ```abnf
/// fragment      = *( pchar / "/" / "?" )
/// ```
#[tracing::instrument(level = "trace")]
fn fragment<'str, E>(input: &'str str) -> IResult<&'str str, Fragment<'str>, E>
where
    E: ParseError<&'str str>,
{
    let (input, frag) = recognize(many1(alt((pchar, one_of("/?")))))(input)?;
    Ok((input, Fragment(frag)))
}

/// Percentage Encoded u32 codepoint.
///
/// ```abnf
/// pct-encoded   = "%" HEXDIG HEXDIG
/// ```
#[tracing::instrument(level = "trace")]
fn pct_encoded<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    let (input, hex) = preceded(nchar('%'), recognize(pair(hexdig, hexdig)))(input)?;
    let value = u32::from_str_radix(hex, 16)
        .map_err(|_| nom::Err::Error(E::from_error_kind(input, ErrorKind::HexDigit)))?;
    let ch = char::from_u32(value).ok_or(nom::Err::Error(E::from_error_kind(
        input,
        ErrorKind::HexDigit,
    )))?;

    Ok((input, ch))
}

/// ```abnf
/// unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
/// ```
#[tracing::instrument(level = "trace")]
fn unreserved<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    alt((alphanumeric, one_of("-._~")))(input)
}

/// ```abnf
/// reserved      = gen-delims / sub-delims
/// ```
#[allow(unused)]
#[tracing::instrument(level = "trace")]
fn reserved<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    alt((gen_delims, sub_delims))(input)
}

/// ```abnf
/// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
/// ```
#[tracing::instrument(level = "trace")]
fn gen_delims<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    one_of(":/?#[]@")(input)
}

///
/// ```abnf
/// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
/// ```
#[tracing::instrument(level = "trace")]
fn sub_delims<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    one_of("!$&'()*+,;=")(input)
}

/// ```abnf
/// alphanumeric = ALPHA / DIGIT
/// ```
#[tracing::instrument(level = "trace")]
fn alphanumeric<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    alt((alpha, digit))(input)
}

/// ABNF Rule for ALPHA
///
/// ```abnf
/// ALPHA = "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M"
///       / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z"
///       / "a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m"
///       / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z"
/// ```
#[tracing::instrument(level = "trace")]
fn alpha<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

/// ABNF Rule for DIGIT
/// ```abnf
/// DIGIT = "0" / "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
/// ```
#[tracing::instrument(level = "trace")]
fn digit<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    one_of("0123456789")(input)
}

/// ABNF Rule for Hex Digits (HEXDIG)
/// ```abnf
/// HEXDIG = DIGIT / "A" / "B" / "C" / "D" / "E" / "F" / "a" / "b" / "c" / "d" / "e" / "f"
/// ```
#[tracing::instrument(level = "trace")]
fn hexdig<'str, E>(input: &'str str) -> IResult<&'str str, char, E>
where
    E: ParseError<&'str str>,
{
    one_of("0123456789ABCDEFabcdef")(input)
}

#[cfg(test)]
mod tests {
    use crate::URI;

    const TEST_URIS: [&str; 18] = [
        "http://example.com",
        "json://example.com:8996/",
        "socket://testuser:testpass@www.example.com",
        "https://example.com/",
        "https://example.com/path/to/thing",
        "https://example.com:8912/path/to/thing",
        "https://example.com/path/to/thing?hi=bye&ho=no",
        "https://example.com/path/to/thing?hi=bye&ho=no#test",
        "docs://example.com/path/to/thing#fraggy",
        "file:///path/to/thing?hi=bye&ho=no",
        "https://john.doe@www.example.com:1234/forum/questions/?tag=networking&order=newest#top",
        "https://john.doe@www.example.com:1234/forum/questions/?tag=networking&order=newest#:~:text=whatever",
        "ldap://[2001:db8::7]/c=GB?objectClass?one",
        "mailto:John.Doe@example.com",
        "news:comp.infosystems.www.servers.unix",
        "tel:+1-816-555-1212",
        "telnet://192.0.2.16:80/",
        "urn:oasis:names:specification:docbook:dtd:xml:4.1.2"
    ];

    #[test]
    #[tracing_test::traced_test]
    fn test_uri_parsing() {
        let mut failures = 0;
        for (idx, str) in TEST_URIS.iter().enumerate() {
            match URI::parse(str) {
                Ok(uri) => {
                    tracing::info!("{idx} '{str}' => {uri:#?}");
                }
                Err(err) => {
                    tracing::error!("{idx} '{str}' {err:?}");
                    failures += 1;
                }
            }
        }
        assert_eq!(failures, 0, "Failures Detected");
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_parsing() {
        let mut failures = 0;
        for (idx, str) in TEST_URIS.iter().enumerate() {
            match URI::parse(str) {
                Ok(uri) => {
                    tracing::info!("{idx} '{str}' => {uri:#?}");
                }
                Err(err) => {
                    tracing::error!("{idx} '{str}' {err:?}");
                    failures += 1;
                }
            }
        }
        assert_eq!(failures, 0, "Failures Detected");
    }
}
