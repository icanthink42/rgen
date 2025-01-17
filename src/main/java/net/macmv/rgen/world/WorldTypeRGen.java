package net.macmv.rgen.world;

import net.minecraft.world.World;
import net.minecraft.world.WorldType;
import net.minecraft.world.biome.BiomeProvider;
import net.minecraft.world.gen.IChunkGenerator;

public class WorldTypeRGen extends WorldType {
  public WorldTypeRGen() {
    super("rgen");
  }

  @Override
  public BiomeProvider getBiomeProvider(World world) {
    return super.getBiomeProvider(world);
  }

  @Override
  public IChunkGenerator getChunkGenerator(World world, String generatorOptions) {
    return new RGenChunkGenerator(world);
  }
}
