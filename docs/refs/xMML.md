# MML: towards a Multiscale Modeling Language


```xml
<model id="suspensionFlow">
  <description>
    Flow with a suspension of particles. The conentration
    of particles affect locally the flow viscosity and the
    particles are advected by the flow.
  </description>
  <submodel id="flow">
    <spacescale dimension="2" dx="1 mm" lx="10 cm" ly="30 cm" />
    <spacescale dt="1 ms" t="1 min" />
    <ports>
      <in id="concentration" operator="C" dt="1 ms" dx="1 mm" />
      <out id="velocity" operator="Oi" dt="10 ms" dx="1 mm" />
    </ports>
  </submodel>
  <submodel id="advectionDiffusion">
    <spacescale dimension="2" dx="1 mm" lx="10 cm" ly="30 cm" />
    <spacescale dt="10 ms" t="1 min" />
    <ports>
      <in id="velocity" operator="C" dt="10 ms" dx="1 mm" />
      <out id="concentration" operator="Oi" dt="10 ms" dx="1 mm" />
    </ports>
  </submodel>
  <coupling from="flow.velocity" to="advectionDiffusion.velocity" />
  <coupling from="advectionDiffusion.concentration" to="flow.concentration">
    <filter kind="timeInterpolation" />
  </coupling>b
</model>
```

