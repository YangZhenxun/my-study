#include "pch.h"
#include "framework.h"
#include "models.h"

void requests::RequestHooksMixin::register_hook(std::string event, std::string hook) {
	/*Properly register a hook.*/

	if (this->hooks.find(event) == this->hooks.end()) {
		throw std::exception(std::format("Unsupported event specified, with event name \"{}\"", event).c_str());
	}

}