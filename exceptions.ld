PROVIDE(nmi         = abort);
PROVIDE(hard_fault  = abort);
PROVIDE(svc         = abort);
PROVIDE(pend_sv     = abort);
PROVIDE(sys_tick    = abort);

ENTRY(reset)
