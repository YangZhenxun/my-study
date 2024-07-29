#include "pch.h"
#include "framework.h"
#include "__init__.h"

HTTPStatus HTTPStatus::__new__(int val) {
	value = val;
	return *this;
}
bool HTTPStatus::is_informational() {
	return 100 <= value && value <= 199;
}
bool HTTPStatus::is_success() {
	return 200 <= value && value <= 299;
}
bool HTTPStatus::is_redirection() {
	return 300 <= value && value <= 399;
}
bool HTTPStatus::is_client_error() {
	return 400 <= value && value <= 499;
}
bool HTTPStatus::is_server_error() {
	return 500 <= value && value <= 599;
}
HTTPStatus::HTTPStatus(int val) {
	__new__(val);
}

HTTPMethod HTTPMethod::__new__(std::string val) {
	value = val;
	return *this;
}

std::string HTTPMethod::__repr__() {
	return "<__main__.HTTPMethod><__main__.HTTPMethod.__repr__>";
}
