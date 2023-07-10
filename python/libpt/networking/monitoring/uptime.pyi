"""
# monitor your network uptime

This method offers a way to monitor your networks/hosts uptime. This is achieved by making
HTTPS requests to a given list of
"""

class UptimeStatus:
    """
    Describes an uptime status
    
    UptimeStatus describes the result of an uptime check.
    """
    """ true if the [`UptimeStatus`] is considered successful"""
    success: bool
    """ the percentage of reachable urls out of the total urls"""
    success_ratio: int
    """ the percentage of reachable urls out of the total urls that need to be reachable in order
    for this [`UptimeStatus`] to be considered a success.
    """
    success_ratio_target: int
    """ the number of reachable [`urls`](UptimeStatus::urls) """
    reachable: int
    """ set a timeout (in ms) """
    timeout: int
    """URL list cant be ported to python, use UptimeStatus.urls()"""
    __urls: ...

    def __init__(self, success_ratio_target: int, url_strs: list[str], timeout: int) -> None:
        """
        create a new UptimeStatus and check it

        `success_ratio_target` should never be more than 100 (it represents a success percentage)
        """
        ...

    def check(self) -> None:
        """
        checks if the stored urls

        Makes the actual https requests and updates fields accordingly.

        This method can block some time, as the web requests are implemented as blocking and 
        executed by the shared library (not in python)
        """
        ...

    def calc_success(self) -> None:
        """
        calculate the success based on the `reachable` and `total`
        
        Calculates the ratio of [reachable]/
        (length of [__urls]).
        
        Calculates a [`success_ratio`] from that, by multiplying with 100, then flooring.
        
        If the [`success_ratio`] is greater than or equal to the [`success_ratio_target`], 
        the [`UptimeStatus`] will be considered a success.
        
        In the special case that no URLs to check for have been provided, the check will be
        considered a success, but the [`success_ratio`] will be `0`.
        
        Note: does not check for networking, use [`check()`] for that.
        """
        ...

    def urls(self) -> list[str]:
        """
        get urls for python
        
        Since [`__urls`] has no python equivalent, return the urls as a `list[str]` in
        Python.
        """
        ...

def continuous_uptime_monitor(success_ratio_target: int, urls: list[str], interval: int, timeout: int) -> None:
    """
    Uptime monitor
    
    This function continuously monitors the uptime of your host/network.
    
    On change of status, an update will be logged at INFO Level, containing
    information on your current status, including timestamps of the last up/down time and durations
    since.
    """
    ...
