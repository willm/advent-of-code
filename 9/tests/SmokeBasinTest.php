<?php declare(strict_types=1);
use PHPUnit\Framework\TestCase;

final class SmokeBasinTest extends TestCase
{
    public function test_make_window(): void
    {
        $input = array(array(1, 0), array(2, 1));
        $window = new Window($input, 0, 0);
        $this->assertEquals(
            $window->get(),
            array(
                array(null, null, null),
                array(null, 1, 0),
                array(null, 2, 1)
            )
        );
    }

    public function test_is_low_point_false(): void
    {
        $input = array(array(3, 0), array(2, 1));
        $window = new Window($input, 0, 0);
        $low_point = $window->is_low_point();
        $this->assertEquals(
            $low_point,
            false
        );
    }

    public function test_is_low_point_true(): void
    {
        $input = array(
            array(1, 0),
            array(2, 1)
        );
        $window = new Window($input, 0, 1);
        $low_point = $window->is_low_point();
        $this->assertEquals(
            $low_point,
            true
        );
    }
    public function test_get_low_points(): void
    {
        $input = array(
            array(1, 0),
            array(2, 1)
        );
        $actual = SmokeBasin::get_low_points($input);
        $this->assertEquals(
            1,
            count($actual)
        );
        $this->assertEquals($actual[0], array(0,1));
    }

    public function test_parse_height_map(): void
    {
        $actual = HeightMap::from_file("./input-test.txt");
        $this->assertEquals(count($actual), 5);
        $this->assertEquals(count($actual[0]), 10);
        $this->assertEquals($actual[0][9], 0);
    }

    public function test_integration_test(): void
    {
        $actual = SmokeBasin::get_low_points_from_file("./input-test.txt");
        var_dump($actual);
    }
}
