namespace PolyrealmBot.Utilities
{
    public static class Extensions
    {
        public static string SanitizeEntity(this string entity)
            => entity.Replace("Entity", string.Empty);
    }
}
