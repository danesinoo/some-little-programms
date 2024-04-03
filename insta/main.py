import instaloader
import json
import os

username = 'danesinoo'
password = 'KarloRoed'

def init():
    L = instaloader.Instaloader()
    L.login(username, password)
    profile = instaloader.Profile.from_username(L.context, username)
    return L, profile

def save_log(actual_followers, followees):
    with open('log.txt', 'w') as f:
        data = json.dumps({'actual_followers': list(actual_followers),
                           'followees': list(followees)})
        f.write(data)

def get_followers(profile):
    followers = set(profile.get_followers())
    username = set()
    for follower in followers:
        username.add(follower.username)
    return username


def get_followees(profile):
    followees = set(profile.get_followees())
    username = set()
    for followee in followees:
        username.add(followee.username)
    return username

def load_log():
    if not os.path.exists('log.txt'):
        return set(), set()
    with open('log.txt', 'r') as f:
        data = json.loads(f.read())
        return set(data['actual_followers']), set(data['followees'])


if __name__ == '__main__':
    L, profile = init()
    actual_followers, followees = load_log()

    new_followers = get_followers(profile)
    new_followees = get_followees(profile)

    print('New followers:')
    for f in new_followers - actual_followers:
        print(f)

    print('New followees:')
    for f in new_followees - followees:
        print(f)

    print('Unfollowers:')
    for f in actual_followers - new_followers:
        print(f)

    print('Not following back:')
    for f in new_followees - new_followers:
        print(f)

    save_log(new_followers, new_followees)
