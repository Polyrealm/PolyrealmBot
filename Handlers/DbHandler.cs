using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using LiteDB;
using PolyrealmBot.Handlers.Entities;
using PolyrealmBot.Utilities;

namespace PolyrealmBot.Handlers
{
    public class DbHandler
    {
        private readonly string _dbName = "./polyrealm.db";

        private readonly ConcurrentDictionary<object, BaseEntity> _cache = new ConcurrentDictionary<object, BaseEntity>();

        public T Get<T>(object id) where T : BaseEntity
        {
            if (_cache.TryGetValue(id, out var cached))
                return (T)cached;

            var collection = GetCollection<T>();
            var get = collection.FindOne(x => x.Id == id);
            if (!(get is null))
                _cache.TryAdd(get.Id, get);
            return get;
        }

        public void Save<T>(T document) where T : BaseEntity
        {
            var collection = GetCollection<T>();
            if (!collection.Exists(x => x.Id == document.Id))
            {
                collection.Insert(document);
                _cache.TryAdd(document.Id, document);
            }
            else
            {
                collection.Update(document);
                _cache.TryUpdate(document.Id, document, null);
            }
        }

        public void Delete<T>(T document) where T : BaseEntity
        {
            var collection = GetCollection<T>();
            collection.Delete(x => x.Id == document.Id);
            _cache.TryRemove(document.Id, out _);
        }

        public void VerifyGuilds(IEnumerable<ulong> guildIds)
        {
            var collection = GetCollection<GuildEntity>();
            var fetchAll = collection.FindAll().Select(x => ulong.Parse($"{x.Id}")).ToHashSet();
            foreach (var guildId in guildIds)
            {
                if (!fetchAll.Contains(guildId))
                {
                    collection.Insert(new GuildEntity
                    {
                        Id = guildId,
                        Prefix = "p!",
                    });
                }
            }
        }

        private LiteCollection<T> GetCollection<T>() where T : BaseEntity
        {
            using (var database = new LiteDatabase(_dbName))
                return database.GetCollection<T>(typeof(T).Name.SanitizeEntity());
        }
    }
}
