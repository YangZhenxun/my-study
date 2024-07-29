#pragma once
#include "pch.h"
#include "framework.h"
#include <iostream>
#include <stdio.h>
#include <string>
#include <tuple>

class HTTPStatus
{
    /*
    HTTP status codes and reason phrases

        Status codes from the following RFCs are all observed :

    *RFC 7231 : Hypertext Transfer Protocol(HTTP / 1.1), obsoletes 2616
        * RFC 6585 : Additional HTTP Status Codes
        * RFC 3229 : Delta encoding in HTTP
        * RFC 4918 : HTTP Extensions for WebDAV, obsoletes 2518
        * RFC 5842 : Binding Extensions to WebDAV
        * RFC 7238 : Permanent Redirect
        * RFC 2295 : Transparent Content Negotiation in HTTP
        * RFC 2774 : An HTTP Extension Framework
        * RFC 7725 : An HTTP Status Code to Report Legal Obstacles
        * RFC 7540 : Hypertext Transfer Protocol Version 2 (HTTP / 2)
        * RFC 2324 : Hyper Text Coffee Pot Control Protocol(HTCPCP / 1.0)
        * RFC 8297 : An HTTP Status Code for Indicating Hints
        * RFC 8470 : Using Early Data in HTTP
        */
public:
	HTTPStatus __new__(int val);
	bool is_informational();
	bool is_success();
	bool is_redirection();
	bool is_client_error();
	bool is_server_error();
	HTTPStatus(int val);

    // informational
	const std::tuple<int, std::string, std::string> CONTINUE = std::make_tuple(100, "Continue", "Request received, please continue");
    const std::tuple<int, std::string, std::string> SWITCHING_PROTOCOLS = std::make_tuple(101, "Switching Protocols", "Switching to new protocol; obey Upgrade header");
    const std::tuple<int, std::string> PROCESSING = std::make_tuple(102, "Processing");
    const std::tuple<int, std::string> EARLY_HINTS = std::make_tuple(103, "Early Hints");

    // success
    const std::tuple<int, std::string, std::string> OK = std::make_tuple(200, "OK", "Request fulfilled, document follows");
    const std::tuple<int, std::string, std::string> CREATED = std::make_tuple(201, "Created", "Document created, URL follows");
    const std::tuple<int, std::string, std::string> ACCEPTED = std::make_tuple(202, "Accepted","Request accepted, processing continues off-line");
    const std::tuple<int, std::string, std::string> NON_AUTHORITATIVE_INFORMATION = std::make_tuple(203,"Non-Authoritative Information", "Request fulfilled from cache");
    const std::tuple<int, std::string, std::string> NO_CONTENT = std::make_tuple(204, "No Content", "Request fulfilled, nothing follows");
    const std::tuple<int, std::string, std::string> RESET_CONTENT = std::make_tuple(205, "Reset Content", "Clear input form for further input");
    const std::tuple<int, std::string, std::string> PARTIAL_CONTENT = std::make_tuple(206, "Partial Content", "Partial content follows");
    const std::tuple<int, std::string> MULTI_STATUS = std::make_tuple(207, "Multi-Status");
    const std::tuple<int, std::string> ALREADY_REPORTED = std::make_tuple(208, "Already Reported");
    const std::tuple<int, std::string> IM_USED = std::make_tuple(226, "IM Used");

    // redirection
    const std::tuple<int, std::string, std::string> MULTIPLE_CHOICES = std::make_tuple(300, "Multiple Choices","Object has several resources -- see URI list");
    const std::tuple<int, std::string, std::string> MOVED_PERMANENTLY = std::make_tuple(301, "Moved Permanently", "Object moved permanently -- see URI list");
    const std::tuple<int, std::string, std::string> FOUND = std::make_tuple(302, "Found", "Object moved temporarily -- see URI list");
    const std::tuple<int, std::string, std::string> SEE_OTHER = std::make_tuple(303, "See Other", "Object moved -- see Method and URL list");
    const std::tuple<int, std::string, std::string> NOT_MODIFIED = std::make_tuple(304, "Not Modified", "Document has not changed since given time");
    const std::tuple<int, std::string, std::string> USE_PROXY = std::make_tuple(305, "Use Proxy", "You must use proxy specified in Location to access this resource");
    const std::tuple<int, std::string, std::string> TEMPORARY_REDIRECT = std::make_tuple(307, "Temporary Redirect", "Object moved temporarily -- see URI list");
    const std::tuple<int, std::string, std::string> PERMANENT_REDIRECT = std::make_tuple(308, "Permanent Redirect", "Object moved permanently -- see URI list");

    // client error
    const std::tuple<int, std::string, std::string> BAD_REQUEST = std::make_tuple(400, "Bad Request", "Bad request syntax or unsupported method");
    const std::tuple<int, std::string, std::string> UNAUTHORIZED = std::make_tuple(401, "Unauthorized", "No permission -- see authorization schemes");
    const std::tuple<int, std::string, std::string> PAYMENT_REQUIRED = std::make_tuple(402, "Payment Required","No payment -- see charging schemes");
    const std::tuple<int, std::string, std::string> FORBIDDEN = std::make_tuple(403, "Forbidden", "Request forbidden -- authorization will not help");
    const std::tuple<int, std::string, std::string> NOT_FOUND = std::make_tuple(404, "Not Found","Nothing matches the given URI");
    const std::tuple<int, std::string, std::string> METHOD_NOT_ALLOWED = std::make_tuple(405, "Method Not Allowed", "Specified method is invalid for this resource");
    const std::tuple<int, std::string, std::string> NOT_ACCEPTABLE = std::make_tuple(406, "Not Acceptable", "URI not available in preferred format");
    const std::tuple<int, std::string, std::string> PROXY_AUTHENTICATION_REQUIRED = std::make_tuple(407, "Proxy Authentication Required", "You must authenticate with this proxy before proceeding");
    const std::tuple<int, std::string, std::string> REQUEST_TIMEOUT = std::make_tuple(408, "Request Timeout", "Request timed out; try again later");
    const std::tuple<int, std::string, std::string> CONFLICT = std::make_tuple(409, "Conflict", "Request conflict");
    const std::tuple<int, std::string, std::string> GONE = std::make_tuple(410, "Gone", "URI no longer exists and has been permanently removed");
    const std::tuple<int, std::string, std::string> LENGTH_REQUIRED = std::make_tuple(411, "Length Required", "Client must specify Content-Length");
    const std::tuple<int, std::string, std::string> PRECONDITION_FAILED = std::make_tuple(412, "Precondition Failed", "Precondition in headers is false");
    const std::tuple<int, std::string, std::string> REQUEST_ENTITY_TOO_LARGE = std::make_tuple(413, "Request Entity Too Large", "Entity is too large");
    const std::tuple<int, std::string, std::string> REQUEST_URI_TOO_LONG = std::make_tuple(414, "Request-URI Too Long", "URI is too long");
    const std::tuple<int, std::string, std::string> UNSUPPORTED_MEDIA_TYPE = std::make_tuple(415, "Unsupported Media Type", "Entity body in unsupported format");
    const std::tuple<int, std::string, std::string> REQUESTED_RANGE_NOT_SATISFIABLE = std::make_tuple(416, "Requested Range Not Satisfiable", "Cannot satisfy request range");
    const std::tuple<int, std::string, std::string> EXPECTATION_FAILED = std::make_tuple(417, "Expectation Failed", "Expect condition could not be satisfied");
    const std::tuple<int, std::string, std::string> IM_A_TEAPOT = std::make_tuple(418, "I\"m a Teapot", "Server refuses to brew coffee because it is a teapot.");
    const std::tuple<int, std::string, std::string> MISDIRECTED_REQUEST = std::make_tuple(421, "Misdirected Request", "Server is not able to produce a response");
    const std::tuple<int, std::string> UNPROCESSABLE_ENTITY = std::make_tuple(422, "Unprocessable Entity");
    const std::tuple<int, std::string> LOCKED = std::make_tuple(423, "Locked");
    const std::tuple<int, std::string> FAILED_DEPENDENCY = std::make_tuple(424, "Failed Dependency");
    const std::tuple<int, std::string> TOO_EARLY = std::make_tuple(425, "Too Early");
    const std::tuple<int, std::string> UPGRADE_REQUIRED = std::make_tuple(426, "Upgrade Required");
    const std::tuple<int, std::string, std::string> PRECONDITION_REQUIRED = std::make_tuple(428, "Precondition Required", "The origin server requires the request to be conditional");
    const std::tuple<int, std::string, std::string> TOO_MANY_REQUESTS = std::make_tuple(429, "Too Many Requests", "The user has sent too many requests in"
        "a given amount of time (\"rate limiting\")");
    const std::tuple<int, std::string, std::string> REQUEST_HEADER_FIELDS_TOO_LARGE = std::make_tuple(431, "Request Header Fields Too Large", "The server is unwilling to process the request because its header "
        "fields are too large");
    const std::tuple<int, std::string, std::string> UNAVAILABLE_FOR_LEGAL_REASONS = std::make_tuple(451,
        "Unavailable For Legal Reasons",
        "The server is denying access to the "
        "resource as a consequence of a legal demand");

    // server errors
    const std::tuple<int, std::string, std::string> INTERNAL_SERVER_ERROR = std::make_tuple(500, "Internal Server Error",
        "Server got itself in trouble");
    const std::tuple<int, std::string, std::string> NOT_IMPLEMENTED = std::make_tuple(501, "Not Implemented",
        "Server does not support this operation");
    const std::tuple<int, std::string, std::string> BAD_GATEWAY = std::make_tuple(502, "Bad Gateway",
        "Invalid responses from another server/proxy");
    const std::tuple<int, std::string, std::string> SERVICE_UNAVAILABLE = std::make_tuple(503, "Service Unavailable",
        "The server cannot process the request due to a high load");
    const std::tuple<int, std::string, std::string> GATEWAY_TIMEOUT = std::make_tuple(504, "Gateway Timeout",
        "The gateway server did not receive a timely response");
    const std::tuple<int, std::string, std::string> HTTP_VERSION_NOT_SUPPORTED = std::make_tuple(505, "HTTP Version Not Supported",
        "Cannot fulfill request");
    const std::tuple<int, std::string> VARIANT_ALSO_NEGOTIATES = std::make_tuple(506, "Variant Also Negotiates");
    const std::tuple<int, std::string> INSUFFICIENT_STORAGE = std::make_tuple(507, "Insufficient Storage");
    const std::tuple<int, std::string> LOOP_DETECTED = std::make_tuple(508, "Loop Detected");
    const std::tuple<int, std::string> NOT_EXTENDED = std::make_tuple(510, "Not Extended");
    const std::tuple<int, std::string, std::string> NETWORK_AUTHENTICATION_REQUIRED = std::make_tuple(511,
        "Network Authentication Required",
        "The client needs to authenticate to gain network access");
private:
	int value;
};

class HTTPMethod
{/*
    HTTP methods and descriptions

Methods from the following RFCs are all observed :

*RFC 7231 : Hypertext Transfer Protocol(HTTP / 1.1), obsoletes 2616
* RFC 5789 : PATCH Method for HTTP
*/
public:
    HTTPMethod __new__(std::string val);
    std::string __repr__();
    const std::tuple<std::string, std::string> CONNECT = std::make_tuple("CONNECT", "Establish a connection to the server.");
    const std::tuple<std::string, std::string> DELETE = std::make_tuple("DELETE", "Remove the target.");
    const std::tuple<std::string, std::string> GET = std::make_tuple("GET", "Retrieve the target.");
    const std::tuple<std::string, std::string> HEAD = std::make_tuple("HEAD", "Same as GET, but only retrieve the status line and header section.");
    const std::tuple<std::string, std::string> OPTIONS = std::make_tuple("OPTIONS", "Describe the communication options for the target.");
    const std::tuple<std::string, std::string> PATCH = std::make_tuple("PATCH", "Apply partial modifications to a target.");
    const std::tuple<std::string, std::string> POST = std::make_tuple("POST", "Perform target-specific processing with the request payload.");
    const std::tuple<std::string, std::string> PUT = std::make_tuple("PUT", "Replace the target with the request payload.");
    const std::tuple<std::string, std::string> TRACE = std::make_tuple("TRACE", "Perform a message loop-back test along the path to the target.");
private:
    std::string value;
};
