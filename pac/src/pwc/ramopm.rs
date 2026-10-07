#[doc = "Register `RAMOPM` reader"]
pub type R = crate::R<RamopmSpec>;
#[doc = "Register `RAMOPM` writer"]
pub type W = crate::W<RamopmSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc RAMOPM\n\nYou can [`read`](crate::Reg::read) this register and get [`ramopm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramopm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamopmSpec;
impl crate::RegisterSpec for RamopmSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ramopm::R`](R) reader structure"]
impl crate::Readable for RamopmSpec {}
#[doc = "`write(|w| ..)` method takes [`ramopm::W`](W) writer structure"]
impl crate::Writable for RamopmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMOPM to value 0x8043"]
impl crate::Resettable for RamopmSpec {
    const RESET_VALUE: u16 = 0x8043;
}
