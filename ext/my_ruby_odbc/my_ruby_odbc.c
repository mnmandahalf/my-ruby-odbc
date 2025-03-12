#include "my_ruby_odbc.h"

VALUE rb_mMyRubyOdbc;

void
Init_my_ruby_odbc(void)
{
  rb_mMyRubyOdbc = rb_define_module("MyRubyOdbc");
}
