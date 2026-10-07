#[doc = "Register `GVBUSCFG` reader"]
pub type R = crate::R<GvbuscfgSpec>;
#[doc = "Register `GVBUSCFG` writer"]
pub type W = crate::W<GvbuscfgSpec>;
#[doc = "Field `VBUSOVEN` reader - desc VBUSOVEN"]
pub type VbusovenR = crate::BitReader;
#[doc = "Field `VBUSOVEN` writer - desc VBUSOVEN"]
pub type VbusovenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSVAL` reader - desc VBUSVAL"]
pub type VbusvalR = crate::BitReader;
#[doc = "Field `VBUSVAL` writer - desc VBUSVAL"]
pub type VbusvalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - desc VBUSOVEN"]
    #[inline(always)]
    pub fn vbusoven(&self) -> VbusovenR {
        VbusovenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc VBUSVAL"]
    #[inline(always)]
    pub fn vbusval(&self) -> VbusvalR {
        VbusvalR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - desc VBUSOVEN"]
    #[inline(always)]
    pub fn vbusoven(&mut self) -> VbusovenW<'_, GvbuscfgSpec> {
        VbusovenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc VBUSVAL"]
    #[inline(always)]
    pub fn vbusval(&mut self) -> VbusvalW<'_, GvbuscfgSpec> {
        VbusvalW::new(self, 7)
    }
}
#[doc = "desc GVBUSCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`gvbuscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gvbuscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GvbuscfgSpec;
impl crate::RegisterSpec for GvbuscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gvbuscfg::R`](R) reader structure"]
impl crate::Readable for GvbuscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gvbuscfg::W`](W) writer structure"]
impl crate::Writable for GvbuscfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GVBUSCFG to value 0"]
impl crate::Resettable for GvbuscfgSpec {}
