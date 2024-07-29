#include <pybind11/pybind11.h>
#include <uv.h>

namespace py = pybind11;

#define PY_UV_HANDLE_FIELDS(x) \
    .def(py::init<>()) \
	.def_readwrite("data", &x::data) \
	.def_readwrite("loop", &x::loop) \
	.def_readwrite("type", &x::type) \
	.def_readwrite("close_cb", &x::close_cb) \
	.def_readwrite("handle_queue", &x::handle_queue) \
	.def_readwrite("endgame_next", &x::endgame_next) \
	.def_readwrite("flags", &x::flags)

PYBIND11_MODULE(pythuv, m) {
	m.def("uv_version_string", &uv_version_string);
	m.def("uv_version", &uv_version);
	typedef py::class_<uv_loop_s> __py_loop_s;
	__py_loop_s py_uv_loop_s(m, "uv_loop_s");
	py_uv_loop_s
		.def(py::init<>())
		.def_readwrite("data", &uv_loop_s::data)
		.def_readwrite("internal_fields", &uv_loop_s::internal_fields)
		.def_readwrite("active_handles", &uv_loop_s::active_handles)
		.def_readwrite("handle_queue", &uv_loop_s::handle_queue)
		.def_readwrite("stop_flag", &uv_loop_s::stop_flag)
		.def_readwrite("iocp", &uv_loop_s::iocp)
		.def_readwrite("time", &uv_loop_s::time)
		.def_readwrite("pending_reqs_tail", &uv_loop_s::pending_reqs_tail)
		.def_readwrite("endgame_handles", &uv_loop_s::endgame_handles)
		.def_readwrite("timer_heap", &uv_loop_s::timer_heap)
		.def_readwrite("prepare_handles", &uv_loop_s::prepare_handles)
		.def_readwrite("check_handles", &uv_loop_s::check_handles)
		.def_readwrite("idle_handles", &uv_loop_s::idle_handles)
		.def_readwrite("next_prepare_handle", &uv_loop_s::next_prepare_handle)
		.def_readwrite("next_check_handle", &uv_loop_s::next_check_handle)
		.def_readwrite("next_idle_handle", &uv_loop_s::next_idle_handle)
		.def_readwrite("active_tcp_streams", &uv_loop_s::active_tcp_streams)
		.def_readwrite("active_udp_streams", &uv_loop_s::active_udp_streams)
		.def_readwrite("wq", &uv_loop_s::wq)
		.def_readwrite("wq_mutex", &uv_loop_s::wq_mutex)
		.def_readwrite("wq_async", &uv_loop_s::wq_async);
	py::class_<uv_loop_s::_active_reqs> py_uv_loop_active_reqs(py_uv_loop_s, "active_reqs");
	py_uv_loop_active_reqs
		.def(py::init<>())
		.def_readwrite("unused", &uv_loop_s::_active_reqs::unused)
		.def_readwrite("count", &uv_loop_s::_active_reqs::count);
	py::enum_<uv_run_mode> py_uv_run_mode(m, "uv_run_mode");
	py_uv_run_mode
		.value("UV_RUN_DEFAULT", UV_RUN_DEFAULT)
		.value("UV_RUN_ONCE", UV_RUN_ONCE)
		.value("UV_RUN_NOWAIT", UV_RUN_NOWAIT)
		.export_values();
	py::class_<uv_handle_s> py_uv_handle_s(m, "uv_handle_s");
	py_uv_handle_s
		PY_UV_HANDLE_FIELDS(uv_handle_s);
	py::class_<uv_handle_s::_u> py_uv_handle_s_u(py_uv_handle_s, "u");
	py_uv_handle_s_u
		.def(py::init<>())
		.def_readwrite("fd", &uv_handle_s::_u::fd);

	typedef py::class_<uv_idle_s> __py_uv_idle_s;
	__py_uv_idle_s py_uv_idle_s(m, "uv_idle_s");
	py_uv_idle_s
		PY_UV_HANDLE_FIELDS(uv_idle_s)
		.def_readwrite("idle_prev", &uv_idle_s::idle_prev)
		.def_readwrite("idle_next", &uv_idle_s::idle_next)
		.def_readwrite("idle_cb", &uv_idle_s::idle_cb);
	py::class_<uv_idle_s::_u> py_uv_loop_s_u(py_uv_loop_s, "u");
	py_uv_loop_s_u
		.def(py::init<>())
		.def_readwrite("fd", &uv_idle_s::_u::fd);
	typedef py::class_<uv_idle_cb> sth;
	sth py_uv_idle_cb(m, "uv_idle_cb");
	py_uv_idle_cb.def(py::init<py::cpp_function*>());
	py::class_<uv_close_cb> py_uv_close_cb(m, "uv_close_cb");
	m.attr("uv_loop_t") = py_uv_loop_s;
	m.attr("uv_idle_t") = py_uv_idle_s;
	m.attr("uv_handle_t") = py_uv_handle_s;
	m.def("uv_default_loop", &uv_default_loop);
	m.def("uv_loop_init", &uv_loop_init);
	m.def("uv_run", &uv_run);
	m.def("uv_loop_close", &uv_loop_close);
	m.def("uv_idle_init", &uv_idle_init);
	m.def("uv_idle_start", [](uv_idle_t *idlei,uv_idle_cb cbi){uv_idle_start(idlei, cbi);});
	m.def("uv_idle_stop", &uv_idle_stop);
}
