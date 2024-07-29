#pragma once
#include "pch.h"
#include "framework.h"
#include <iostream>
#include <regex>
#include <string>
#include <format>
#include <set>
#include <iconv.h>
#include <list>
#include <fstream>
#include <istream>
#include <exception>

const int HTTP_PORT = 80;
const int HTTPS_PORT = 443;

const std::string _UNKNOWN = "UNKNOWN";

// connection states
const std::string _CS_IDLE = "Idle";
const std::string _CS_REQ_STARTED = "Request-started";
const std::string _CS_REQ_SENT = "Request-sent";

const int _MAXLINE = 65536;
const int _MAXHEADERS = 100;

// Header name / value ABNF(http://tools.ietf.org/html/rfc7230#section-3.2)
//
// VCHAR = % x21 - 7E
// obs - text = % x80 - FF
// header - field = field - name ":" OWS field - value OWS
// field - name = token
// field - value = *(field - content / obs - fold)
// field - content = field - vchar[1 * (SP / HTAB) field - vchar]
// field - vchar = VCHAR / obs - text
//
// obs - fold = CRLF 1 * (SP / HTAB)
//                ; obsolete line folding
//                ; see Section 3.2.4

// token = 1 * tchar
//
// tchar = "!" / "#" / "$" / "%" / "&" / "'" / "*"
//                / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
//                / DIGIT / ALPHA
//                ; any VCHAR, except delimiters
//
// VCHAR defined in http ://tools.ietf.org/html/rfc5234#appendix-B.1

// the patterns for both name and value are more lenient than RFC
// definitions to allow for backwards compatibility

auto _is_legal_header_name(std::string string) {
	std::smatch smatch1;
	std::regex rx1(std::string("[^ :\s][^ :\r\n] * "));
	if (std::regex_match(string, smatch1, rx1))
	{
		return std::string(std::format("<re.March object; match={}>", smatch1.str()));
	}
	else {
		return std::string("None");
	}
}

auto _is_illegal_header_value (std::string string) {
	std::smatch smatch1;
	std::regex rx1(std::string("\n(?![ \t])|\r(?![ \t\n])"));
	if (std::regex_match(string, smatch1, rx1))
	{
		return std::string(std::format("<re.March object; match={}>", smatch1.str()));
	}
	else {
		return std::string("None");
	}
}

// These characters are not allowed within HTTP URL paths.
//  See https ://tools.ietf.org/html/rfc3986#section-3.3 and the
//  https ://tools.ietf.org/html/rfc3986#appendix-A pchar definition.
// Prevents CVE - 2019 - 9740.  Includes control characters such as \r\n.
// We don't restrict chars above \x7f as putrequest() limits us to ASCII.

std::regex _contains_disallowed_url_pchar_re(std::string("[\x00-\x20\x7f]"));
// Arguably only these _should_ allowed :
//  _is_allowed_url_pchars_re = re.compile(r"^[/!$&'()*+,;=:@%a-zA-Z0-9._~-]+$")
// We are more lenient for assumed real world compatibility purposes.

// These characters are not allowed within HTTP method names
// to prevent http header injection.
std::regex _contains_disallowed_method_pchar_re(std::string("[\x00-\x1f]"));

// We always set the Content - Length header for these methods because some
// servers will otherwise respond with a 411
std::set<std::string> _METHODS_EXPECTING_BODY{ std::string("PATCH"), std::string("POST"), std::string("PUT") };

std::string _encode(std::string data, std::string name=std::string("data")) {
	/*Call data.encode("latin-1") but show a better error message.*/
	iconv_t iconvdat1 = iconv_open("latin-1", "");
	if (iconvdat1 == 0) {
		std::cerr << std::format("{} ({:<20}) is not valid Latin-1. Use {}.encode('utf-8') "
            "if you want to send it encoded in UTF-8.", name, data, name) << std::endl;
		throw "";
	}
	size_t in_len = data.size();
	size_t out_len = 255;
	char outdat[255];
	char* data_in = new char[in_len];
	strcpy_s(data_in, sizeof(data.c_str()), data.c_str());
	memset(outdat, 0, out_len);
	char* indat = data_in;
	char* out_dat = outdat;
	size_t ins_len = in_len;
	size_t iconvdat2 = iconv(iconvdat1, &data_in, &ins_len, &out_dat, &out_len);
	if (iconvdat2 == -1) {
		std::cerr << std::format("{} ({:<20}) is not valid Latin-1. Use {}.encode('utf-8') "
			"if you want to send it encoded in UTF-8.", name, data, name) << std::endl;
		throw "";
	}
	size_t iconvclosedat = iconv_close(iconvdat1);
	if (iconvclosedat == -1) {
		std::cerr << std::format("{} ({:<20}) is not valid Latin-1. Use {}.encode('utf-8') "
			"if you want to send it encoded in UTF-8.", name, data, name) << std::endl;
		throw "";
	}
	std::string out_data(outdat);
	return out_data;
	delete []data_in;
}

//TODO:
//class HTTPMessage: public email.message.Message
//    std::list getallmatchingheaders(){};

class HTTPException :public std::exception {
public:
	HTTPException(std::string _data, int exit_num) noexcept :
		exception(_data.c_str(), exit_num) {};
};
class LineTooLone : public HTTPException
{
public:
	LineTooLone(std::string line_type) noexcept :
		HTTPException(std::format("got more than {} bytes when reading {}.", _MAXLINE, line_type).c_str(), 1) {};
};

std::list<std::string> _read_headers(std::fstream fp) {
	/*Reads potential header lines into a list from a file pointer.

    Length of line is limited by _MAXLINE, and number of
    headers is limited by _MAXHEADERS.
	*/
	std::list<std::string> headers;
	while (true)
	{
		std::string line;
		getline(fp, line);
		if (size(line) > _MAXLINE) {
			throw(LineTooLone(std::string("header line")));
		}
		headers.emplace_back(line);
		if (size(headers) > _MAXHEADERS) {
			throw(HTTPException(std::format("got more than {} headers", _MAXHEADERS), 1));
		}
		if (line == std::string("\n") || line == std::string("\r\n") || line == std::string("")) {
			break;
		}
	}
	return headers;
}
