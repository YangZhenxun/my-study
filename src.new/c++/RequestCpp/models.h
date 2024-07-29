#pragma once
#include "pch.h"
#include "framework.h"
#include <string>
#include <map>
#include <exception>
#include <format>

namespace requests {
	class RequestHooksMixin {
	public:
		std::map<std::string, std::string> hooks;
		void register_hook(std::string event, std::string hook);
		bool deregister_hook(std::string event, std::string hook);
	};
}