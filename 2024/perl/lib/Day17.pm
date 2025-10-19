use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Register {
    field $a : param : reader : writer;
    field $b : param : reader : writer;
    field $c : param : reader : writer;

    method clone() {
        return Register->new( a => $a, b => $b, c => $c );
    }

    method combo_operand($v) {
        return $v if $v <= 3;
        return $a if $v == 4;
        return $b if $v == 5;
        return $c if $v == 6;
        die "Invalid operand: $v";
    }
}

class Computer {
    field $register : param : reader;
    field $program  : param : reader;

    method run( $a = undef ) {
        my $r = $register->clone();
        $r->set_a( $a // $r->a() );
        my $i       = 0;
        my @outputs = ();
        while ( $i < $program->@* ) {
            my ( $opcode, $operand ) = $program->@[ $i, $i + 1 ];
            if ( $opcode == 0 ) {
                $r->set_a( $r->a() >> $r->combo_operand($operand) );
            }
            elsif ( $opcode == 1 ) {
                $r->set_b( $r->b() ^ $operand );
            }
            elsif ( $opcode == 2 ) {
                $r->set_b( $r->combo_operand($operand) % 8 );
            }
            elsif ( $opcode == 3 && $r->a > 0 ) {
                $i = $operand - 2;
            }
            elsif ( $opcode == 4 ) {
                $r->set_b( $r->b() ^ $r->c() );
            }
            elsif ( $opcode == 5 ) {
                push @outputs, $r->combo_operand($operand) % 8;
            }
            elsif ( $opcode == 6 ) {
                $r->set_b( $r->a >> $r->combo_operand($operand) );
            }
            elsif ( $opcode == 7 ) {
                $r->set_c( $r->a >> $r->combo_operand($operand) );
            }
            $i += 2;
        }
        return @outputs;
    }
}

class Solution : isa(Base) {

    method parse($lines) {
        my ($a)       = $lines->[0] =~ /Register A:\s*(\d+)/;
        my ($b)       = $lines->[1] =~ /Register B:\s*(\d+)/;
        my ($c)       = $lines->[2] =~ /Register C:\s*(\d+)/;
        my ($program) = $lines->[4] =~ /Program:\s*([\d,]+)/;
        return Computer->new(
            register => Register->new( a => $a, b => $b, c => $c ),
            program  => [ split /,/, $program ],
        );
    }

    method part1() {
        my $input = $self->input();
        return join ',', $input->run();
    }

    method part2() {
        my $input = $self->input();
        return find_initial_value( 0, 0, $input );
    }

    sub find_initial_value( $a, $i, $computer ) {
        my @output = $computer->run($a);
        if ( $i == 0 || $output[0] == $computer->program->[ -$i ] ) {
            if ( $i == scalar $computer->program->@* ) {
                return $a;
            }
            for my $n ( 0 .. 7 ) {
                my $res = find_initial_value( $a * 8 + $n, $i + 1, $computer );
                return $res if defined $res;
            }
        }
        return undef;
    }
}
