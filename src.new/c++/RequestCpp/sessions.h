#pragma once

#include "pch.h"
#include "framework.h"
#include <chrono>

namespace requests {
	std::chrono::high_resolution_clock::time_point preferred_clock = std::chrono::high_resolution_clock::now();
}