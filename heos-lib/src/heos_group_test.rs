///
/// @package heos-dial
///
/// @file HEOS group tests
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

#[cfg(test)]
mod heos_device_test {
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use crate::heos_group::HeosGroup;

    #[fixture]
    fn heos_group() -> HeosGroup {
        HeosGroup::new(env!("TEST_GROUP_NAME"), "")
    }

    #[rstest]
    fn should_clone_and_compare_with_itself(heos_group: HeosGroup) {
        let cloned = heos_group.clone();

        assert_eq!(heos_group, cloned);
    }
}
